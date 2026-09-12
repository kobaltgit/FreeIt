# 💡 Реестр фич и идей FreeIt (Feature List)

> Центральный журнал планирования, фиксации и отслеживания внедрения функциональности продукта **FreeIt**.

---

## 📋 Реестр плановых и реализованных возможностей

| ID | Категория | Название фичи | Описание | Приоритет | Статус | Документ / План |
| :--- | :--- | :--- | :--- | :---: | :---: | :--- |
| **FEAT-001** | Core Win32 | **Restart Manager Engine** | Поиск процессов (PID, заголовок, имя) через `rstrtmgr.dll` | High | Completed | [FEAT-001_plan.md](file:///d:/Projects/active/FreeIt/features/FEAT-001_plan.md) |
| **FEAT-002** | Release Engine | **Graceful Process Release** | Мягкая остановка через `WM_CLOSE` / `WM_QUERYENDSESSION` | High | Planned | [PLAN.md](file:///d:/Projects/active/FreeIt/PLAN.md#этап-1-исследование-и-настройка-win32-ядра-rust-engine) |
| **FEAT-003** | Safety & Undo | **Safe Vault & Backup** | Теневой бэкап заблокированного файла в `%LOCALAPPDATA%` до удаления | High | Planned | [PLAN.md](file:///d:/Projects/active/FreeIt/PLAN.md#этап-2-система-безопасности-и-теневых-бэкапов-safe-vault) |
| **FEAT-004** | Automation | **Watcher & Auto-Free** | Отложенное авто-удаление элемента на вотчере `notify` | Medium | Planned | [PLAN.md](file:///d:/Projects/active/FreeIt/PLAN.md#этап-3-отложенный-вотчер-watcher--auto-free) |
| **FEAT-005** | Ecosystem | **PeekIt Read-Only Preview** | Просмотр заблокированного файла по нажатию `Space` | Medium | Planned | [PLAN.md](file:///d:/Projects/active/FreeIt/PLAN.md#этап-4-пользовательский-интерфейс-svelte-5-runes--tauri-v2) |
| **FEAT-006** | UI & UX | **Smart Risk Indicator** | Цветовая маркировка степени риска процессов (🟢/🟡/🔴) | High | Planned | [PLAN.md](file:///d:/Projects/active/FreeIt/PLAN.md#этап-4-пользовательский-интерфейс-svelte-5-runes--tauri-v2) |
| **FEAT-007** | Integration | **StashIt Drag-to-Unlock** | Разблокировка файлов при перетаскивании в виджет StashIt | Medium | Planned | [PLAN.md](file:///d:/Projects/active/FreeIt/PLAN.md#этап-5-системная-интеграция-и-контекстное-меню-windows) |
| **FEAT-008** | Integration | **Windows Shell Context Menu** | Пункт *"Разблокировать в FreeIt"* в меню ПКМ Explorer (файлы, папки, диски) + Single Instance IPC | High | Completed | [FEAT-008](file:///d:/Projects/active/FreeIt/src-tauri/src/engine/settings.rs) |
| **FEAT-009** | Ecosystem | **Recycle Bin & MiniBin Sync** | Удаление в Корзину (`SHFileOperationW` + `FOF_ALLOWUNDO`) и нотификация MiniBin (`SHChangeNotify`) | High | Completed | [file_ops.rs](file:///d:/Projects/active/FreeIt/src-tauri/src/engine/file_ops.rs) |
| **FEAT-010** | AI & Inspector | **Gemini AI Explainer & Settings** | Экспертный разбор файла/блокировки (Что за файл, Кто блокирует, Совет) + выбор моделей из API | High | Completed | [gemini.rs](file:///d:/Projects/active/FreeIt/src-tauri/src/engine/gemini.rs) |
| **FEAT-011** | UI & UX | **Human-Friendly Names & Window Mode** | Человекопонятные имена процессов, полноценный оконный режим без костылей с Esc и трей-меню | High | Completed | [App.svelte](file:///d:/Projects/active/FreeIt/src/App.svelte) |
| **FEAT-012** | Core & UI | **Unlock, Rename & Delete Actions** | Панель быстрых действий: Разблокировать всё (`unlock_all`), Переименовать (`rename_target`), Удалить в Корзину (`delete_to_trash`) | High | Completed | [App.svelte](file:///d:/Projects/active/FreeIt/src/App.svelte) |
| **FEAT-013** | Localization & AI | **Multilingual Support & Strict AI Language Alignment** | Двуязычность RU/EN, переключатель языка и тем (Dark/Light/System Acrylic), извлечение Win32 PE метаданных и строгая генерация ответов Gemini на выбранном языке | High | Completed | [i18n.ts](file:///d:/Projects/active/FreeIt/src/lib/i18n.ts) |

---

## 📜 Хронология внедрения возможностей (Feature Worklog)

| Дата / Время | ID Фичи | Изменение / Статус | Исполнитель | Результат |
| :--- | :--- | :--- | :--- | :--- |
| 2026-09-12 | ALL | Первичная архитектура и проектирование реестра фич | Kobalt Agent | Сформирован план разработки |
| 2026-09-12 | FEAT-008, FEAT-010, FEAT-011 | Контекстное меню Explorer, настройки AI и нормальный оконный режим с Esc | Kobalt Agent | Релизы собраны в `.output/` |
| 2026-09-12 | FEAT-009, FEAT-010 | Внедрение ИИ-консультанта Gemini + нативное удаление в Корзину с поддержкой MiniBin | Kobalt Agent | Удалены все выдуманные зависимости от Undoit, оставлена чистая Корзина |
| 2026-09-12 | FEAT-012 | Полноценная триада действий: Разблокировать, Переименовать, Удалить в Корзину | Kobalt Agent | Кнопка удаления видна всегда, инлайн-панель переименования, разблокировка в 1 клик |



