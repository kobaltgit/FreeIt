<!-- markdownlint-disable MD033 MD041 -->
<p align="center">
  <img src="src-tauri/icons/128x128.png" width="96" height="96" alt="FreeIt Logo" />
  <h1 align="center">FreeIt</h1>
  <strong>Нативная утилита разблокировки и безопасного удаления файлов для Windows 10 & 11</strong><br/>
  <em>Native, ultra-lightweight file & folder unlocker for Windows 10 & 11</em>
</p>

<p align="center">
  <a href="https://github.com/kobaltgit/FreeIt/releases/latest"><img src="https://img.shields.io/github/v/release/kobaltgit/FreeIt?color=38bdf8&label=Latest%20Release" alt="Latest Release" /></a>
  <a href="https://kobaltgit.github.io/FreeIt/"><img src="https://img.shields.io/badge/Website-Flutter%20Web-02569B.svg?logo=flutter" alt="Live Website" /></a>
  <img src="https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011-0078D6.svg?logo=windows" alt="Windows 10/11" />
  <img src="https://img.shields.io/badge/Rust-2021%20Edition-DEA584.svg?logo=rust" alt="Rust 2021" />
  <img src="https://img.shields.io/badge/Tauri-v2.0-FFC131.svg?logo=tauri" alt="Tauri v2" />
  <img src="https://img.shields.io/badge/Frontend-Svelte%205%20(Runes)-FF3E00.svg?logo=svelte" alt="Svelte 5" />
  <img src="https://img.shields.io/badge/RAM-%3C%2015%20MB-34d399.svg" alt="Low RAM" />
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="MIT License" /></a>
</p>

<p align="center">
  <a href="#-о-проекте">🇷🇺 Русский</a> • <a href="#-about-the-project">🇬🇧 English</a> • <a href="#-экосистема-kobalt-tools">🌐 Экосистема</a>
</p>

---

## 🇷🇺 О проекте

**FreeIt** — сверхлегковесная нативная утилита для Windows 10 & 11, входящая в экосистему системных инструментов **Kobalt Tools** ([StashIt](https://github.com/kobaltgit/StashIt), [MiniBin](https://github.com/kobaltgit/minibin), [PolyShift](https://github.com/kobaltgit/polyshift), [PeekIt](https://github.com/kobaltgit/peekit)).

Она мгновенно устраняет распространенную проблему Windows — невозможность удалить, переименовать или переместить файл/папку из-за ошибки *«Файл уже используется другим процессом»*. FreeIt производит анализ блокирующих процессов через нативный Win32 API и предоставляет безопасный инструмент разблокировки и удаления.

### ⚡ Сравнение с аналогами

| Параметр | FreeIt | IObit Unlocker | LockHunter |
| :--- | :--- | :--- | :--- |
| **Стек технологий** | **Rust + Tauri v2 + Svelte 5** | C++ / MFC (Legacy UI) | C++ / Win32 |
| **ОЗУ в фоне** | **< 15 МБ** | ~45–80 МБ | ~40–70 МБ |
| **Холодный запуск** | **~50 мс** | ~300–600 мс | ~400–700 мс |
| **Размер дистрибутива** | **~3–5 МБ** | > 15 МБ (с рекламой) | > 8 МБ |
| **Теневой бэкап и отмена** | **Есть (Safe Vault + Корзина)** | ❌ Нет | ❌ Частично |
| **Права администратора** | **Не требуются (чистый HKCU)** | Требуются UAC | Требуются UAC |

### 🎯 Ключевые возможности

- 🔍 **Restart Manager API:** Мгновенный поиск всех PID, заголовков и путей процессов, удерживающих файл или папку.
- 🛡️ **Graceful Release & Safe Vault:** Мягкое завершение программ с сохранением данных и теневой бэкап перед принудительным закрытием.
- ⏳ **Watcher & Auto-Free:** Отложенный режим ожидания — удаление элемента в ту миллисекунду, когда блокирующий процесс сам завершит работу.
- 👁️ **Предпросмотр через PeekIt:** Просмотр содержимого заблокированного элемента по нажатию `Space`.
- 🎨 **Адаптивный Fluent-дизайн:** Тёмная и светлая темы с поддержкой размытия и цветов Windows 11.
- 🌐 **Двуязычный интерфейс:** Полная локализация (Русский / Английский).
- 🔒 **Приватность и безопасность:** Работает на 100% локально, без телеметрии и скрытых процессов.

### 📥 Установка и загрузка

Скачайте актуальную версию со [страницы последнего релиза](https://github.com/kobaltgit/FreeIt/releases/latest):

- **Инсталлятор (`.msi` / `Setup.exe`):** Быстрая установка в профиль пользователя без прав администратора (чистый HKCU).
- **Portable версия (`.zip`):** Запуск без установки, можно носить на флешке.

---

## 🇬🇧 About the Project

**FreeIt** is an ultra-lightweight, native Windows 10 & 11 utility and part of the **Kobalt Tools** desktop ecosystem ([StashIt](https://github.com/kobaltgit/StashIt), [MiniBin](https://github.com/kobaltgit/minibin), [PolyShift](https://github.com/kobaltgit/polyshift), [PeekIt](https://github.com/kobaltgit/peekit)).

It instantly resolves a common Windows issue where files or folders cannot be deleted or moved because they are locked by background processes (*"The file is in use by another process"*). Powered by native Win32 APIs and Rust, FreeIt delivers instant lock diagnostics and safe file releasing.

### ⚡ Key Benchmarks

| Metric | FreeIt | IObit Unlocker | LockHunter |
| :--- | :--- | :--- | :--- |
| **Tech Stack** | **Rust + Tauri v2 + Svelte 5** | C++ / MFC (Legacy UI) | C++ / Win32 |
| **Idle RAM** | **< 15 MB** | ~45–80 MB | ~40–70 MB |
| **Cold Launch** | **~50 ms** | ~300–600 ms | ~400–700 ms |
| **Installer Size** | **~3–5 MB** | > 15 MB (ad-supported) | > 8 MB |
| **Shadow Backup & Undo** | **Yes (Safe Vault + Recycle Bin)** | ❌ No | ❌ Partial |
| **Admin Rights** | **Zero Admin (pure HKCU)** | Required | Required |

### 🎯 Core Features

- 🔍 **Restart Manager API:** Instant lookup of all locking PIDs, window titles, and file handles.
- 🛡️ **Graceful Release & Safe Vault:** Soft-close application signals + shadow backups before forced handle release.
- ⏳ **Watcher & Auto-Free:** Delayed auto-delete mode triggering the exact millisecond a file is released by a process.
- 👁️ **PeekIt Integration:** Read-only preview of locked files by pressing `Space`.
- 🎨 **Fluent Acrylic UI:** Dark and light modes matching Windows 11 aesthetics.
- 🌐 **Bilingual:** Full English and Russian localization.
- 🔒 **100% Local & Private:** No telemetry, no cloud upload, zero bloat.

### 📥 Installation & Download

Download the latest version from [GitHub Releases](https://github.com/kobaltgit/FreeIt/releases/latest):

- **Installer (`.msi` / `Setup.exe`):** Fast user-mode setup, no UAC prompts.
- **Portable (`.zip`):** Unpack and run anywhere.

---

## 🛠️ Сборка и разработка / Development

```bash
# 1. Установка зависимостей фронтенда
npm install

# 2. Запуск в режиме разработки (Hot Reload)
npm run tauri dev

# 3. Сборка релизного установщика
npm run tauri build
```

---

## 🌐 Экосистема Kobalt Tools

| Проект | Описание | Стек | Ссылки |
| :--- | :--- | :--- | :--- |
| 📥 **StashIt** | Плавающий карман Drag-and-Drop (Dropover / Yoink для Windows) | Rust + Tauri v2 + Svelte 5 | [Repo](https://github.com/kobaltgit/StashIt) • [Web](https://kobaltgit.github.io/StashIt/) |
| 🗑️ **MiniBin** | Умная корзина в системном трее с Flyout-интерфейсом | Rust + Tauri v2 + Svelte 5 | [Repo](https://github.com/kobaltgit/minibin) • [Web](https://kobaltgit.github.io/minibin/) |
| 🌐 **PolyShift** | HUD-помощник и контекстный перевод у курсора с Gemini AI | Rust + Tauri v2 + Svelte 5 | [Repo](https://github.com/kobaltgit/polyshift) • [Web](https://kobaltgit.github.io/polyshift/) |
| 👁️ **PeekIt** | Мгновенный предпросмотр файлов по клавише Space | Rust + Tauri v2 + Svelte 5 | [Repo](https://github.com/kobaltgit/peekit) • [Web](https://kobaltgit.github.io/PeekIt/) |
| 🧩 **PeekIt Plugins** | Официальный реестр и SDK веб-плагинов для PeekIt | TypeScript + Web SDK | [Repo](https://github.com/kobaltgit/peekit-plugins) • [Web](https://kobaltgit.github.io/peekit-plugins/) |
| 🎨 **kobalt_ui** | Общая библиотека UI компонентов (шапка, футер, релизы) | Flutter Web (Dart) | [Repo](https://github.com/kobaltgit/kobalt_ui) |

---

## 📄 Лицензия / License

Распространяется под лицензией **MIT**. Подробнее в файле [LICENSE](LICENSE).