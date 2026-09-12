use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub display_name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingResult {
    pub success: bool,
    pub latency_ms: u64,
    pub models: Vec<ModelInfo>,
    pub error: Option<String>,
}

#[derive(Deserialize)]
struct RawModel {
    name: String,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    description: Option<String>,
    #[serde(rename = "supportedGenerationMethods")]
    supported_generation_methods: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct ModelsResponse {
    models: Option<Vec<RawModel>>,
}

pub async fn list_models(api_key: &str) -> Result<Vec<ModelInfo>, String> {
    if api_key.trim().is_empty() {
        return Err("API ключ не указан".into());
    }

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Ошибка создания HTTP-клиента: {}", e))?;

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models?key={}",
        api_key.trim()
    );

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Ошибка подключения к Google API: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        if status.as_u16() == 400 || status.as_u16() == 403 {
            return Err("Неверный API-ключ Gemini или доступ ограничен".into());
        }
        return Err(format!("Ошибка Google API ({status}): {body}"));
    }

    let data: ModelsResponse = resp
        .json()
        .await
        .map_err(|e| format!("Ошибка разбора списка моделей: {}", e))?;

    let mut result = Vec::new();
    if let Some(raw_models) = data.models {
        for m in raw_models {
            let methods = m.supported_generation_methods.unwrap_or_default();
            if methods.iter().any(|method| method == "generateContent") {
                let id = m.name.trim_start_matches("models/").to_string();
                let display_name = m.display_name.unwrap_or_else(|| id.clone());
                let description = m.description.unwrap_or_default();
                result.push(ModelInfo {
                    id,
                    display_name,
                    description,
                });
            }
        }
    }

    // Сортировка как в PolyShift: сначала флагманские и быстрые 2.5-flash, 2.0-flash, 1.5-flash
    result.sort_by(|a, b| {
        let score = |name: &str| -> i32 {
            if name.contains("2.5-flash") {
                100
            } else if name.contains("2.0-flash") {
                90
            } else if name.contains("1.5-flash") {
                80
            } else if name.contains("1.5-pro") {
                70
            } else {
                10
            }
        };
        score(&b.id).cmp(&score(&a.id))
    });

    Ok(result)
}

pub async fn ping_gemini(api_key: String) -> PingResult {
    let start = Instant::now();
    match list_models(&api_key).await {
        Ok(models) => PingResult {
            success: true,
            latency_ms: start.elapsed().as_millis() as u64,
            models,
            error: None,
        },
        Err(err) => PingResult {
            success: false,
            latency_ms: start.elapsed().as_millis() as u64,
            models: Vec::new(),
            error: Some(err),
        },
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiExplanation {
    pub file_purpose: String,
    pub lock_reason: String,
    pub recommendation: String,
    pub is_safe: bool,
}

#[derive(Deserialize)]
struct GenerateResponsePart {
    text: Option<String>,
}

#[derive(Deserialize)]
struct GenerateResponseContent {
    parts: Option<Vec<GenerateResponsePart>>,
}

#[derive(Deserialize)]
struct GenerateResponseCandidate {
    content: Option<GenerateResponseContent>,
}

#[derive(Deserialize)]
struct GenerateResponse {
    candidates: Option<Vec<GenerateResponseCandidate>>,
}

pub async fn explain_file_and_lock(
    api_key: &str,
    model: &str,
    lang: &str,
    file_path: &str,
    processes: &[crate::engine::ProcessInfo],
) -> Result<AiExplanation, String> {
    let is_en = lang.trim().to_lowercase().starts_with("en");

    if api_key.trim().is_empty() {
        return if is_en {
            Err("Gemini API key is not configured. Go to Settings tab to enter your API key.".into())
        } else {
            Err("API-ключ Gemini не настроен. Перейдите во вкладку Настройки для ввода ключа.".into())
        };
    }

    let active_model = if model.trim().is_empty() {
        "gemini-2.5-flash"
    } else {
        model.trim()
    };

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| {
            if is_en {
                format!("Failed to create HTTP client: {}", e)
            } else {
                format!("Ошибка создания HTTP-клиента: {}", e)
            }
        })?;

    let mut proc_details = String::new();
    if processes.is_empty() {
        if is_en {
            proc_details.push_str("No active processes hold handles to this file (handles are free).\n");
        } else {
            proc_details.push_str("Процессы не удерживают файл (дескрипторы свободны).\n");
        }
    } else {
        for (i, p) in processes.iter().enumerate() {
            let title = p.window_title.as_deref().unwrap_or(if is_en { "no window" } else { "нет окна" });
            let exe = p.exe_path.as_deref().unwrap_or(&p.name);
            let friendly = p.friendly_name.as_deref().unwrap_or(&p.name);
            let pe_desc = p
                .exe_path
                .as_deref()
                .and_then(crate::engine::friendly_names::get_pe_file_description);

            if is_en {
                proc_details.push_str(&format!(
                    "{}. Process: {} (PID: {}, Friendly name: \"{}\", Exe path: {}, Window title: \"{}\", Risk level: {:?})\n",
                    i + 1, p.name, p.pid, friendly, exe, title, p.risk_level
                ));
                if let Some(pe) = pe_desc {
                    proc_details.push_str(&format!("   Executable PE metadata (Win32): \"{}\"\n", pe));
                }
            } else {
                proc_details.push_str(&format!(
                    "{}. Процесс: {} (PID: {}, Понятное имя: \"{}\", Exe: {}, Заголовок окна: \"{}\", Уровень риска: {:?})\n",
                    i + 1, p.name, p.pid, friendly, exe, title, p.risk_level
                ));
                if let Some(pe) = pe_desc {
                    proc_details.push_str(&format!("   Метаданные исполняемого файла (Win32 PE): \"{}\"\n", pe));
                }
            }
        }
    }

    let prompt = if is_en {
        format!(
            r#"You are an expert Windows systems engineer and cybersecurity analyst for FreeIt (part of the Kobalt Tools ecosystem).
A user requested an analysis of a locked/in-use file system object.

Target object path: "{}"
Locking processes:
{}

Official Kobalt Tools ecosystem reference (do NOT hallucinate other purposes for these apps):
- StashIt: Floating cursor drop-pocket / shelf for collecting, holding, and dragging multiple files, text, and media.
- PolyShift: Smart multilingual keyboard layout switcher and AI writing assistant.
- MiniBin: Sleek compact Recycle Bin indicator and control in the Windows notification tray.
- PeekIt: Lightning-fast file preview triggered by pressing the Space key (macOS QuickLook style).
- FreeIt: Fast utility for unlocking, renaming, and safely deleting locked/held files and folders in Windows.

Generate a clear, concise, technically accurate response STRICTLY IN ENGLISH.
Format your answer STRICTLY as valid JSON without markdown formatting, without code fences, and without extra commentary:
{{
  "file_purpose": "Brief (1-2 sentences in English): What this file/folder is, its file format, which application/service it belongs to, and what role it serves.",
  "lock_reason": "Brief (1-2 sentences in English): Why and by which process this file handle is currently held, whether active write/read operations are ongoing or if it's simply opened.",
  "recommendation": "Brief (1-2 sentences in English): Specific actionable advice for the user — is it safe to terminate/unlock this process, should the program be closed normally via its window/tray, or should the file NOT be deleted.",
  "is_safe": true or false (true if terminating the locking process and freeing/deleting the file is safe for Windows stability; false if it is a critical system service or unsaved work would be lost)
}}"#,
            file_path, proc_details
        )
    } else {
        format!(
            r#"Ты — встроенный системный эксперт Windows и специалист по безопасности утилиты FreeIt (экосистема Kobalt Tools).
Пользователь обратился за анализом заблокированного объекта файловой системы.

Путь к объекту: "{}"
Блокирующие процессы:
{}

Справочник утилит экосистемы Kobalt Tools (НЕ выдумывай другие назначения для этих программ):
- StashIt: Плавающий буфер-карман у курсора мыши для сбора, удержания и перетаскивания файлов/текста/картинок.
- PolyShift: Умный переключатель раскладки клавиатуры и ИИ-помощник при наборе текста.
- MiniBin: Компактный значок Корзины в системном трее Windows для быстрого просмотра и очистки.
- PeekIt: Быстрый предпросмотр файлов по нажатию клавиши Пробел (Space) в стиле macOS QuickLook.
- FreeIt: Утилита для разблокировки, переименования и безопасного удаления занятых файловой системой файлов и папок.

Сформируй понятный, лаконичный, технически грамотный ответ СТРОГО НА РУССКОМ ЯЗЫКЕ.
Формат ответа СТРОГО валидный JSON без markdown-разметки, без блоков кода и без пояснений:
{{
  "file_purpose": "Кратко (1-2 предложения на русском): что это за файл/папка, формат, к каческому приложению или службе Windows относится, какую роль выполняет.",
  "lock_reason": "Кратко (1-2 предложения на русском): почему и кем именно сейчас удерживается дескриптор, идет ли запись данных или просто файл открыт в программе.",
  "recommendation": "Кратко (1-2 предложения на русском): конкретный совет пользователю — можно ли безопасно снять задачу процесса, нужно ли закрыть программу штатно через окно/трей, или объект удалять категорически нельзя.",
  "is_safe": true или false (true - если завершение процесса и удаление файла безопасно для работы Windows; false - если это важный системный процесс или потеряются несохраненные данные)
}}"#,
            file_path, proc_details
        )
    };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        active_model,
        api_key.trim()
    );

    let payload = serde_json::json!({
        "contents": [
            {
                "parts": [
                    { "text": prompt }
                ]
            }
        ],
        "generationConfig": {
            "temperature": 0.2,
            "maxOutputTokens": 800
        }
    });

    let resp = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            if is_en {
                format!("Failed to send request to Gemini API: {}", e)
            } else {
                format!("Ошибка отправки запроса к Gemini API: {}", e)
            }
        })?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return if is_en {
            Err(format!("Google API returned error ({status}): {body}"))
        } else {
            Err(format!("Google API вернул ошибку ({status}): {body}"))
        };
    }

    let gen_resp: GenerateResponse = resp
        .json()
        .await
        .map_err(|e| {
            if is_en {
                format!("Failed to parse generation response: {}", e)
            } else {
                format!("Ошибка разбора ответа генерации: {}", e)
            }
        })?;

    let text = gen_resp
        .candidates
        .and_then(|c| c.into_iter().next())
        .and_then(|c| c.content)
        .and_then(|c| c.parts)
        .and_then(|p| p.into_iter().next())
        .and_then(|p| p.text)
        .ok_or_else(|| {
            if is_en {
                "Gemini returned an empty response".to_string()
            } else {
                "Gemini вернул пустой ответ".to_string()
            }
        })?;

    // Очищаем markdown code fence если модель вернула ```json ... ```
    let cleaned = text.trim();
    let json_str = if cleaned.starts_with("```json") {
        cleaned.strip_prefix("```json").unwrap_or(cleaned)
    } else if cleaned.starts_with("```") {
        cleaned.strip_prefix("```").unwrap_or(cleaned)
    } else {
        cleaned
    };
    let json_str = json_str.strip_suffix("```").unwrap_or(json_str).trim();

    if let Ok(explanation) = serde_json::from_str::<AiExplanation>(json_str) {
        Ok(explanation)
    } else {
        // Fallback если модель вернула чистый текст
        if is_en {
            Ok(AiExplanation {
                file_purpose: "File analysis performed by AI".to_string(),
                lock_reason: "File is held by the detected process(es)".to_string(),
                recommendation: text,
                is_safe: processes.iter().all(|p| p.risk_level != "danger"),
            })
        } else {
            Ok(AiExplanation {
                file_purpose: "Анализ файла выполнен ИИ".to_string(),
                lock_reason: "Файл удерживается обнаруженными процессами".to_string(),
                recommendation: text,
                is_safe: processes.iter().all(|p| p.risk_level != "danger"),
            })
        }
    }
}
