# 🤖 Правила разработки проекта FreeIt (Kobalt Tools Standard)

Проект входит в экосистему **Kobalt Tools** и подчиняется строгим стандартам качества, производительности и дизайна.

## 🚩 Ключевые требования для AI-агентов

1. **Производительность и память:**
   - Ядро на Rust 2021 + Tauri v2 + Win32 API.
   - Потребление оперативной памяти строго **до 15 МБ RAM**.
   - Не использовать Electron или тяжелые зависимости.

2. **Фронтенд на Svelte 5:**
   - Строго использовать Runes: `$state`, `$derived`, `$effect`, `$props`.
   - Запрещено использовать устаревший синтаксис Svelte 4 (`let count = 0; export let title; $: computed;`).
   - Стили: Fluent Acrylic (`backdrop-filter: blur`), адаптация под системную тему Windows 10/11.

3. **Система отслеживания багов и фич (`.agents/rules/`):**
   - Обязательно соблюдать протоколы из `.agents/rules/bug_tracking_protocol.md` и `.agents/rules/feature_tracking_protocol.md`.
   - При обнаружении любого бага регистрировать инцидент в [`bugs/buglist.md`](file:///d:/Projects/active/FreeIt/bugs/buglist.md) и вести оперативный лог в [`bugs/worklog.md`](file:///d:/Projects/active/FreeIt/bugs/worklog.md).
   - При проектировании и внедрении новых фич регистрировать их в [`features/featurelist.md`](file:///d:/Projects/active/FreeIt/features/featurelist.md) и отслеживать прогресс в [`features/worklog.md`](file:///d:/Projects/active/FreeIt/features/worklog.md).

4. **Сайт-презентация `website/` (Flutter Web):**
   - При добавлении или изменении ключевых возможностей приложения обязательно обновляйте сайт в `website/lib/widgets/`.
   - Сохраняйте двуязычность (RU/EN) в `website/lib/i18n.dart`.

5. **Релизы и сборка:**
   - Сборка нативных релизов осуществляется через GitHub Actions (`release.yml`).
   - Все изменения документировать по эталону в `README.md` и `PLAN.md`.