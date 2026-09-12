<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { translations, type Language } from "./i18n";

  interface ModelInfo {
    id: string;
    display_name: string;
    description: string;
  }

  export interface AppSettings {
    gemini_api_key: string;
    gemini_model: string;
    auto_start: boolean;
    safe_vault: boolean;
    tray_icon_style: "white" | "cyan";
    explorer_context_menu: boolean;
    language: "ru" | "en";
    theme: "dark" | "light" | "system";
  }

  interface PingResult {
    success: boolean;
    latency_ms: number;
    models: ModelInfo[];
    error: string | null;
  }

  let {
    onBack,
    onSettingsChanged,
  }: {
    onBack: () => void;
    onSettingsChanged?: (s: AppSettings) => void;
  } = $props();

  let settings = $state<AppSettings>({
    gemini_api_key: "",
    gemini_model: "gemini-2.5-flash",
    auto_start: false,
    safe_vault: true,
    tray_icon_style: "white",
    explorer_context_menu: true,
    language: "ru",
    theme: "dark",
  });

  // Локализация интерфейса настроек
  let currentLang = $derived((settings.language as Language) || "ru");
  let t = $derived(translations[currentLang] || translations.ru);

  // Модели заполняются ИСКЛЮЧИТЕЛЬНО из Google AI API по ключу
  let availableModels = $state<ModelInfo[]>([]);
  let isLoadingModels = $state(false);
  let modelsError = $state<string | null>(null);

  let showApiKey = $state(false);
  let isCheckingKey = $state(false);
  let pingStatus = $state<{ ok: boolean; latency?: number; msg: string } | null>(null);
  let isSaving = $state(false);
  let saveStatus = $state<string | null>(null);

  onMount(() => {
    loadSettings();
  });

  async function loadSettings() {
    try {
      const data = await invoke<AppSettings>("get_settings");
      settings = data;
      if (settings.gemini_api_key && settings.gemini_api_key.trim()) {
        fetchDynamicModels(settings.gemini_api_key.trim());
      }
    } catch (err) {
      console.error("Failed to load settings:", err);
    }
  }

  function handleLanguageChange() {
    onSettingsChanged?.(settings);
  }

  function handleThemeChange() {
    onSettingsChanged?.(settings);
  }

  async function fetchDynamicModels(key: string) {
    if (!key.trim()) {
      availableModels = [];
      return;
    }
    isLoadingModels = true;
    modelsError = null;
    try {
      const models = await invoke<ModelInfo[]>("fetch_models", { apiKey: key.trim() });
      if (models && models.length > 0) {
        availableModels = models;
        // Если текущая выбранная модель не входит в полученный список, выставляем первую доступную
        if (!models.some((m) => m.id === settings.gemini_model)) {
          settings.gemini_model = models[0].id;
        }
      } else {
        modelsError = currentLang === "en"
          ? "API returned no supported text generation models"
          : "API не вернуло поддерживаемых моделей генерации текста";
      }
    } catch (err: any) {
      modelsError = typeof err === "string" ? err : err?.message || (currentLang === "en" ? "Failed to fetch models" : "Ошибка загрузки моделей");
    } finally {
      isLoadingModels = false;
    }
  }

  async function testApiKey() {
    const key = settings.gemini_api_key.trim();
    if (!key) {
      pingStatus = {
        ok: false,
        msg: currentLang === "en" ? "Please enter an API key to test" : "Введите API-ключ для проверки",
      };
      return;
    }

    isCheckingKey = true;
    pingStatus = null;
    try {
      const res = await invoke<PingResult>("ping_gemini", { apiKey: key });
      if (res.success) {
        pingStatus = {
          ok: true,
          latency: res.latency_ms,
          msg: currentLang === "en"
            ? `Connected (${res.latency_ms} ms) • Found ${res.models.length} models in API`
            : `Подключено (${res.latency_ms} мс) • Найдено ${res.models.length} моделей в API`,
        };
        availableModels = res.models;
        if (res.models.length > 0 && !res.models.some((m) => m.id === settings.gemini_model)) {
          settings.gemini_model = res.models[0].id;
        }
      } else {
        pingStatus = {
          ok: false,
          msg: res.error || (currentLang === "en" ? "Connection error to Gemini API" : "Ошибка подключения к Google Gemini API"),
        };
      }
    } catch (err: any) {
      pingStatus = {
        ok: false,
        msg: `${currentLang === "en" ? "Test error" : "Сбой проверки"}: ${err}`,
      };
    } finally {
      isCheckingKey = false;
    }
  }

  function openAiStudio() {
    window.open("https://aistudio.google.com/app/apikey", "_blank");
  }

  async function handleSave() {
    isSaving = true;
    saveStatus = null;
    try {
      await invoke("save_settings", { settings });
      onSettingsChanged?.(settings);
      saveStatus = t.settingsSavedOk;
      setTimeout(() => {
        saveStatus = null;
      }, 2500);
    } catch (err: any) {
      saveStatus = `${currentLang === "en" ? "Save error" : "Ошибка сохранения"}: ${err}`;
    } finally {
      isSaving = false;
    }
  }
</script>

<div class="settings-container">
  <div class="settings-scroll">
    <!-- Секция: Внешний вид и язык -->
    <div class="section-card">
      <div class="section-header">
        <span class="section-icon">🌐</span>
        <div class="section-title-group">
          <h3>{t.settingsAppearance}</h3>
          <span class="section-subtitle">Interface Language & Theme</span>
        </div>
      </div>

      <!-- Язык интерфейса -->
      <div class="setting-item">
        <label for="app-language">{t.settingsLanguage}</label>
        <select
          id="app-language"
          bind:value={settings.language}
          onchange={handleLanguageChange}
          class="styled-select"
        >
          <option value="ru">{t.settingsLanguageRu}</option>
          <option value="en">{t.settingsLanguageEn}</option>
        </select>
      </div>

      <!-- Тема оформления -->
      <div class="setting-item" style="margin-top: 8px;">
        <label for="app-theme">{t.settingsTheme}</label>
        <select
          id="app-theme"
          bind:value={settings.theme}
          onchange={handleThemeChange}
          class="styled-select"
        >
          <option value="dark">{t.settingsThemeDark}</option>
          <option value="light">{t.settingsThemeLight}</option>
          <option value="system">{t.settingsThemeSystem}</option>
        </select>
      </div>
    </div>

    <!-- Секция ИИ: строго выбор моделей из Google API как в PolyShift -->
    <div class="section-card">
      <div class="section-header">
        <span class="section-icon">✨</span>
        <div class="section-title-group">
          <h3>{t.settingsAiSection}</h3>
          <span class="section-subtitle">{t.settingsAiSubtitle}</span>
        </div>
      </div>

      <div class="setting-item">
        <label for="gemini-key">{t.settingsApiKey}</label>
        <div class="input-with-actions">
          <div class="input-wrap">
            <input
              id="gemini-key"
              type={showApiKey ? "text" : "password"}
              placeholder={t.settingsApiKeyPlaceholder}
              bind:value={settings.gemini_api_key}
              onchange={() => fetchDynamicModels(settings.gemini_api_key)}
              class="styled-input"
            />
            <button
              type="button"
              class="toggle-eye-btn"
              title={showApiKey ? "Hide key" : "Show key"}
              onclick={() => (showApiKey = !showApiKey)}
            >
              {#if showApiKey}
                <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path>
                  <line x1="1" y1="1" x2="23" y2="23"></line>
                </svg>
              {:else}
                <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
                  <circle cx="12" cy="12" r="3"></circle>
                </svg>
              {/if}
            </button>
          </div>

          <button
            type="button"
            class="action-check-btn"
            disabled={isCheckingKey || !settings.gemini_api_key.trim()}
            onclick={testApiKey}
            title={t.settingsBtnTestKey}
          >
            {#if isCheckingKey}
              <div class="btn-spinner"></div>
            {:else}
              <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>
              </svg>
            {/if}
          </button>
        </div>

        {#if pingStatus}
          <div class="ping-banner" class:ping-ok={pingStatus.ok} class:ping-fail={!pingStatus.ok}>
            <span>{pingStatus.ok ? "🟢" : "🔴"}</span>
            <span>{pingStatus.msg}</span>
          </div>
        {/if}

        <div class="hint-row">
          <span class="hint-text">
            <code>{t.settingsKeySharedHint}</code>
          </span>
          <button type="button" class="link-btn" onclick={openAiStudio}>
            {t.settingsAiStudioLink}
          </button>
        </div>
      </div>

      <!-- Выбор моделей, загружаемых исключительно из Google API -->
      <div class="setting-item">
        <div class="label-with-refresh">
          <label for="gemini-model">{t.settingsModel}</label>
          <button
            type="button"
            class="refresh-models-btn"
            title={t.settingsModelRefresh}
            disabled={isLoadingModels || !settings.gemini_api_key.trim()}
            onclick={() => fetchDynamicModels(settings.gemini_api_key)}
          >
            {#if isLoadingModels}
              <div class="btn-spinner"></div>
            {:else}
              <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="23 4 23 10 17 10"></polyline>
                <polyline points="1 20 1 14 7 14"></polyline>
                <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
              </svg>
            {/if}
          </button>
        </div>

        {#if isLoadingModels}
          <div class="models-loading-box">
            <span>⏳ {t.settingsModelLoading}</span>
          </div>
        {:else if availableModels.length > 0}
          <div class="select-wrap">
            <select id="gemini-model" bind:value={settings.gemini_model} class="styled-select">
              {#each availableModels as model}
                <option value={model.id}>
                  {model.display_name || model.id} ({model.id})
                </option>
              {/each}
            </select>
          </div>
          {#if availableModels.find((m) => m.id === settings.gemini_model)}
            <span class="model-desc">
              {availableModels.find((m) => m.id === settings.gemini_model)?.description}
            </span>
          {/if}
          <div class="models-meta-count">
            ✓ {t.settingsModelsCount.replace("{count}", availableModels.length.toString())}
          </div>
        {:else}
          <div class="models-empty-box">
            {#if modelsError}
              <span class="error-text">⚠️ {modelsError}</span>
            {:else}
              <span>ℹ️ {t.settingsModelsEmpty}</span>
            {/if}
          </div>
        {/if}
      </div>
    </div>

    <!-- Секция Интеграция с Windows и Проводником -->
    <div class="section-card">
      <div class="section-header">
        <span class="section-icon">🪟</span>
        <div class="section-title-group">
          <h3>{t.settingsWinSection}</h3>
          <span class="section-subtitle">{t.settingsWinSubtitle}</span>
        </div>
      </div>

      <!-- Пункт контекстного меню Проводника -->
      <div class="toggle-row">
        <div class="toggle-label">
          <span>{t.settingsContextMenu}</span>
          <small>{t.settingsContextMenuSub}</small>
        </div>
        <input
          type="checkbox"
          bind:checked={settings.explorer_context_menu}
          class="styled-checkbox"
        />
      </div>

      <!-- Автозапуск -->
      <div class="toggle-row">
        <div class="toggle-label">
          <span>{t.settingsAutostart}</span>
          <small>{t.settingsAutostartSub}</small>
        </div>
        <input type="checkbox" bind:checked={settings.auto_start} class="styled-checkbox" />
      </div>

      <!-- Safe Vault -->
      <div class="toggle-row">
        <div class="toggle-label">
          <span>{t.settingsSafeVault}</span>
          <small>{t.settingsSafeVaultSub}</small>
        </div>
        <input type="checkbox" bind:checked={settings.safe_vault} class="styled-checkbox" />
      </div>

      <!-- Стиль иконки трея -->
      <div class="setting-item" style="margin-top: 6px;">
        <label for="tray-style">{t.settingsTrayStyle}</label>
        <select id="tray-style" bind:value={settings.tray_icon_style} class="styled-select">
          <option value="white">{t.settingsTrayWhite}</option>
          <option value="cyan">{t.settingsTrayCyan}</option>
        </select>
      </div>
    </div>
  </div>

  <!-- Нижняя панель действий настроек -->
  <div class="settings-actions">
    {#if saveStatus}
      <span class="save-status-badge">{saveStatus}</span>
    {/if}
    <div class="btn-group">
      <button class="back-btn" onclick={onBack} title={t.settingsBackBtn}>
        <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <line x1="19" y1="12" x2="5" y2="12"></line>
          <polyline points="12 19 5 12 12 5"></polyline>
        </svg>
      </button>
      <button class="save-btn" onclick={handleSave} disabled={isSaving} title={t.settingsSaveBtn}>
        {#if isSaving}
          <div class="btn-spinner"></div>
        {:else}
          <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
            <polyline points="17 21 17 13 7 13 7 21"></polyline>
            <polyline points="7 3 7 8 15 8"></polyline>
          </svg>
        {/if}
      </button>
    </div>
  </div>
</div>

<style>
  .settings-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: 12px;
  }

  .settings-scroll {
    display: flex;
    flex-direction: column;
    gap: 12px;
    flex: 1;
    overflow-y: auto;
    padding-right: 4px;
  }

  .settings-scroll::-webkit-scrollbar {
    width: 4px;
  }

  .settings-scroll::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.15);
    border-radius: 4px;
  }

  .section-card {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .section-header {
    display: flex;
    align-items: center;
    gap: 10px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    padding-bottom: 8px;
  }

  .section-icon {
    font-size: 18px;
  }

  .section-title-group h3 {
    margin: 0;
    font-size: 14px;
    font-weight: 600;
    color: #fff;
  }

  .section-subtitle {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.5);
  }

  .setting-item {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .setting-item label {
    font-size: 12px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.85);
  }

  .label-with-refresh {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .refresh-models-btn {
    width: 28px;
    height: 28px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: 1px solid rgba(0, 188, 212, 0.35);
    color: #00bcd4;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .refresh-models-btn:hover:not(:disabled) {
    background: rgba(0, 188, 212, 0.15);
    border-color: #00bcd4;
    color: #ffffff;
    transform: scale(1.05);
  }

  .refresh-models-btn:active:not(:disabled) {
    transform: scale(0.95);
  }

  .refresh-models-btn:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .input-with-actions {
    display: flex;
    gap: 8px;
  }

  .input-wrap {
    position: relative;
    flex: 1;
    display: flex;
    align-items: center;
  }

  .styled-input {
    width: 100%;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 6px;
    padding: 8px 34px 8px 10px;
    color: #fff;
    font-size: 12px;
    outline: none;
    transition: border-color 0.2s;
  }

  .styled-input:focus {
    border-color: #00bcd4;
  }

  .toggle-eye-btn {
    position: absolute;
    right: 6px;
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    padding: 4px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    transition: color 0.15s ease;
  }

  .toggle-eye-btn:hover {
    color: #ffffff;
  }

  .action-check-btn {
    width: 36px;
    height: 36px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 188, 212, 0.15);
    border: 1px solid rgba(0, 188, 212, 0.4);
    color: #00bcd4;
    border-radius: 6px;
    cursor: pointer;
    flex-shrink: 0;
    transition: all 0.2s ease;
  }

  .action-check-btn:hover:not(:disabled) {
    background: rgba(0, 188, 212, 0.25);
    border-color: #00bcd4;
    color: #ffffff;
    transform: scale(1.05);
  }

  .action-check-btn:active:not(:disabled) {
    transform: scale(0.95);
  }

  .action-check-btn:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .ping-banner {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    border-radius: 4px;
    font-size: 11px;
    margin-top: 4px;
  }

  .ping-ok {
    background: rgba(76, 175, 80, 0.15);
    border: 1px solid rgba(76, 175, 80, 0.3);
    color: #81c784;
  }

  .ping-fail {
    background: rgba(244, 67, 54, 0.15);
    border: 1px solid rgba(244, 67, 54, 0.3);
    color: #e57373;
  }

  .hint-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 2px;
  }

  .hint-text {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.4);
  }

  .hint-text code {
    color: rgba(0, 188, 212, 0.8);
  }

  .link-btn {
    background: transparent;
    border: none;
    color: #00bcd4;
    font-size: 11px;
    cursor: pointer;
    padding: 0;
    text-decoration: underline;
  }

  .select-wrap {
    position: relative;
    width: 100%;
  }

  .styled-select {
    width: 100%;
    background: #1e1e1e;
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 6px;
    padding: 8px 10px;
    color: #fff;
    font-size: 12px;
    outline: none;
    cursor: pointer;
  }

  .styled-select:focus {
    border-color: #00bcd4;
  }

  .model-desc {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.6);
    line-height: 1.4;
  }

  .models-loading-box,
  .models-empty-box {
    padding: 10px;
    border-radius: 6px;
    background: rgba(0, 0, 0, 0.2);
    border: 1px dashed rgba(255, 255, 255, 0.15);
    font-size: 11px;
    color: rgba(255, 255, 255, 0.6);
  }

  .models-meta-count {
    font-size: 11px;
    color: #81c784;
    margin-top: 2px;
  }

  .error-text {
    color: #e57373;
  }

  .toggle-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 4px 0;
  }

  .toggle-label {
    display: flex;
    flex-direction: column;
  }

  .toggle-label span {
    font-size: 12px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.85);
  }

  .toggle-label small {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.5);
  }

  .styled-checkbox {
    width: 16px;
    height: 16px;
    accent-color: #00bcd4;
    cursor: pointer;
  }

  .settings-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-top: 1px solid rgba(255, 255, 255, 0.08);
    padding-top: 10px;
  }

  .save-status-badge {
    font-size: 11px;
    color: #81c784;
  }

  .btn-group {
    display: flex;
    gap: 8px;
    margin-left: auto;
  }

  .back-btn {
    width: 36px;
    height: 36px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.14);
    color: #cbd5e1;
    border-radius: 6px;
    cursor: pointer;
    transition: background 0.15s ease, border-color 0.15s ease, color 0.15s ease, transform 0.1s ease;
  }

  .back-btn:hover {
    background: rgba(255, 255, 255, 0.14);
    border-color: rgba(255, 255, 255, 0.3);
    color: #ffffff;
    transform: scale(1.05);
  }

  .back-btn:active {
    transform: scale(0.96);
  }

  .save-btn {
    width: 36px;
    height: 36px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: #00bcd4;
    border: none;
    color: #0c1824;
    border-radius: 6px;
    cursor: pointer;
    transition: filter 0.15s ease, transform 0.1s ease;
  }

  .save-btn:hover:not(:disabled) {
    filter: brightness(1.18);
    transform: scale(1.05);
  }

  .save-btn:active:not(:disabled) {
    transform: scale(0.96);
  }

  .save-btn:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .btn-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.25);
    border-top-color: currentColor;
    border-radius: 50%;
    animation: spin 0.75s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
