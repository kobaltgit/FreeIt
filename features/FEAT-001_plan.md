# План внедрения: FEAT-001 Restart Manager Engine

> Копия утвержденного плана реализации из `implementation_plan.md` в соответствии с протоколом `.agents/rules/feature_tracking_protocol.md`.

## Цель
Реализация базового поискового движка на Win32 Restart Manager API (`rstrtmgr.dll`) для моментального и легковесного обнаружения процессов, удерживающих файлы и папки.

## Шаги реализации:
1. **Настройка окружения сборки**:
   - Создать `src-tauri/build.rs` с вызовом `tauri_build::build()`.
   - Обновить `src-tauri/Cargo.toml`: добавить `tauri-plugin-shell = "2"`, расширить фичи `windows-sys` (`Win32_System_RestartManager`, `Win32_System_Threading`, `Win32_System_ProcessStatus`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`).
2. **Брендинг и системные иконки**:
   - `icon.svg` для тёмной темы (`#FFFFFF`) с адаптивным CSS медиа-запросом для светлой темы (`#00BCD4`).
   - `icon-light.svg` в цвете циан (`#00BCD4`) для светлой темы и светлых интерфейсов.
   - Генерация системных иконок для Tauri v2 (`32x32.png`, `128x128.png`, `icon.ico`) в `src-tauri/icons/`.
3. **Модули Rust Engine**:
   - `src-tauri/src/engine/mod.rs`
   - `src-tauri/src/engine/restart_manager.rs`: `RmStartSession`, `RmRegisterResources`, `RmGetList`, `RmEndSession`.
   - `src-tauri/src/engine/risk.rs`: классификатор уровня риска (`safe`, `warning`, `danger`).
4. **Интеграция с Tauri**:
   - Регистрация команды `get_locking_processes(path: String) -> Result<Vec<ProcessInfo>, String>`.
5. **Тестирование и верификация**:
   - Тест с захватом дескриптора тестового файла и детектом PID.
