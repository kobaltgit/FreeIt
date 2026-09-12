export type Language = "ru" | "en";

export interface Translations {
  // App Header & Tabs
  appName: string;
  tabInspector: string;
  tabSettings: string;
  btnMinimize: string;
  btnCloseTray: string;

  // Drop zone & Input
  dropZoneDrag: string;
  dropZoneDragSub: string;
  dropZoneIdle: string;
  dropZoneIdleSub: string;
  pathPlaceholder: string;
  btnPickFile: string;
  btnPickFolder: string;
  btnPasteClipboard: string;
  btnInspect: string;

  // Status & Badges
  statusWaiting: string;
  statusLocked: string;
  statusFree: string;
  statusIdleMsg: string;
  statusInspectingMsg: string;
  statusFreeMsg: string;
  statusLockedMsg: string;
  statusErrorMsg: string;

  // Quick Action buttons (tooltips)
  actionUnlockAll: string;
  actionUnlockAllDisabled: string;
  actionRename: string;
  actionDeleteTrash: string;
  actionDeleteTrashLocked: string;
  actionAiExplainer: string;

  // Inline Rename
  renamePlaceholder: string;
  renameApply: string;
  renameCancel: string;
  renameSuccess: string;
  renameError: string;

  // Confirmation dialogs
  confirmDeleteTrash: string;
  confirmDeleteTrashLocked: string;
  confirmKillDanger: string;

  // Process list
  processTitle: string;
  processCount: string;
  processPid: string;
  processRiskSafe: string;
  processRiskWarning: string;
  processRiskDanger: string;
  processRiskSafeDesc: string;
  processRiskWarningDesc: string;
  processRiskDangerDesc: string;
  processServiceBadge: string;
  processKillBtn: string;
  processKillSuccess: string;
  processKillError: string;

  // AI Card
  aiTitle: string;
  aiAnalyzing: string;
  aiFilePurpose: string;
  aiLockReason: string;
  aiRecommendation: string;
  aiSafeVerdict: string;
  aiUnsafeVerdict: string;

  // Settings
  settingsTitle: string;
  settingsAppearance: string;
  settingsLanguage: string;
  settingsLanguageRu: string;
  settingsLanguageEn: string;
  settingsTheme: string;
  settingsThemeDark: string;
  settingsThemeLight: string;
  settingsThemeSystem: string;
  settingsAiSection: string;
  settingsAiSubtitle: string;
  settingsApiKey: string;
  settingsApiKeyPlaceholder: string;
  settingsBtnTestKey: string;
  settingsKeySharedHint: string;
  settingsAiStudioLink: string;
  settingsModel: string;
  settingsModelRefresh: string;
  settingsModelLoading: string;
  settingsModelsCount: string;
  settingsModelsEmpty: string;
  settingsWinSection: string;
  settingsWinSubtitle: string;
  settingsContextMenu: string;
  settingsContextMenuSub: string;
  settingsAutostart: string;
  settingsAutostartSub: string;
  settingsSafeVault: string;
  settingsSafeVaultSub: string;
  settingsTrayStyle: string;
  settingsTrayWhite: string;
  settingsTrayCyan: string;
  settingsSaveBtn: string;
  settingsBackBtn: string;
  settingsSavedOk: string;
}

export const translations: Record<Language, Translations> = {
  ru: {
    appName: "FreeIt",
    tabInspector: "Инспектор файлов и процессов",
    tabSettings: "Настройки программы и параметры ИИ",
    btnMinimize: "Свернуть окно",
    btnCloseTray: "Свернуть в трей (Esc)",

    dropZoneDrag: "Отпустите файл или папку для разблокировки!",
    dropZoneDragSub: "Win32 Restart Manager перехватит дескрипторы",
    dropZoneIdle: "Перетащите сюда заблокированный файл или папку",
    dropZoneIdleSub: "Окно не исчезает при потере фокуса • Меню ПКМ в Проводнике",
    pathPlaceholder: "C:\\Путь\\к\\заблокированному\\файлу\\или\\папке...",
    btnPickFile: "Выбрать файл через Проводник Windows",
    btnPickFolder: "Выбрать папку через Проводник Windows",
    btnPasteClipboard: "Вставить путь из буфера обмена",
    btnInspect: "Проверить блокировку файла или папки (Enter)",

    statusWaiting: "Ожидание",
    statusLocked: "🔒 Заблокирован",
    statusFree: "🟢 Свободен",
    statusIdleMsg: "Перетащите файл, укажите путь или выберите через кнопки",
    statusInspectingMsg: "Анализ дескрипторов Win32 Restart Manager...",
    statusFreeMsg: "Файл свободен и не удерживается процессами",
    statusLockedMsg: "Обнаружено процессов, удерживающих файл/папку: {count}",
    statusErrorMsg: "Ошибка анализа: {err}",

    actionUnlockAll: "Разблокировать: завершить все удерживающие процессы ({count})",
    actionUnlockAllDisabled: "Объект свободен, разблокировка не требуется",
    actionRename: "Переименовать файл или папку (с автоматической разблокировкой)",
    actionDeleteTrash: "Переместить в Корзину Windows",
    actionDeleteTrashLocked: "Разблокировать и переместить в Корзину Windows",
    actionAiExplainer: "Экспертный разбор Gemini AI: назначение файла, причина блокировки и совет",

    renamePlaceholder: "Новое имя файла или папки...",
    renameApply: "Применить",
    renameCancel: "Отмена",
    renameSuccess: "Объект успешно переименован в «{name}»",
    renameError: "Ошибка переименования: {err}",

    confirmDeleteTrash: "Переместить в Корзину «{path}»?",
    confirmDeleteTrashLocked: "Объект заблокирован процессами ({count}). Разблокировать и переместить в Корзину «{path}»?",
    confirmKillDanger: "ВНИМАНИЕ: «{name}» — системный процесс Windows! Завершение может привести к сбою или перезагрузке системы. Вы точно уверены?",

    processTitle: "Блокирующие процессы",
    processCount: "Удерживают дескриптор: {count}",
    processPid: "PID: {pid}",
    processRiskSafe: "Безопасно",
    processRiskWarning: "Внимание",
    processRiskDanger: "Опасно",
    processRiskSafeDesc: "Обычное приложение. Завершение безопасно для Windows.",
    processRiskWarningDesc: "Приложение пользователя. Возможно несохранение открытых данных.",
    processRiskDangerDesc: "Критический системный компонент Windows. Завершение запрещено или крайне опасно!",
    processServiceBadge: "Служба",
    processKillBtn: "Завершить процесс (Снять блокировку)",
    processKillSuccess: "Процесс PID {pid} успешно завершен",
    processKillError: "Не удалось завершить PID {pid}: {err}",

    aiTitle: "Экспертный анализ Gemini AI",
    aiAnalyzing: "ИИ анализирует дескрипторы, метаданные PE и контекст файла...",
    aiFilePurpose: "Назначение файла:",
    aiLockReason: "Причина блокировки:",
    aiRecommendation: "Рекомендация:",
    aiSafeVerdict: "Безопасно для системы Windows",
    aiUnsafeVerdict: "Внимание: Системный файл или риск потери данных",

    settingsTitle: "Настройки FreeIt",
    settingsAppearance: "Внешний вид и язык",
    settingsLanguage: "Язык интерфейса",
    settingsLanguageRu: "Русский (RU)",
    settingsLanguageEn: "English (EN)",
    settingsTheme: "Тема оформления",
    settingsThemeDark: "Тёмная (Dark Acrylic)",
    settingsThemeLight: "Светлая (Light Acrylic)",
    settingsThemeSystem: "Системная (Windows)",
    settingsAiSection: "Интеллект Gemini AI",
    settingsAiSubtitle: "Kobalt Tools Ecosystem • Общий ключ реестра с PolyShift",
    settingsApiKey: "API-ключ Google Gemini",
    settingsApiKeyPlaceholder: "AIzaSy...",
    settingsBtnTestKey: "Проверить подключение к Google AI API и измерить отклик (ping)",
    settingsKeySharedHint: "Ключ хранится в HKCU\\Software\\KobaltTools\\API",
    settingsAiStudioLink: "Получить ключ в Google AI Studio ↗",
    settingsModel: "Модель ИИ (загружается из API)",
    settingsModelRefresh: "Обновить список моделей из API Google",
    settingsModelLoading: "Запрос актуального списка моделей из Google API...",
    settingsModelsCount: "В вашем аккаунте доступно моделей: {count}",
    settingsModelsEmpty: "Введите API-ключ выше и нажмите «⚡ Проверить», чтобы получить список моделей непосредственно из Google API.",
    settingsWinSection: "Интеграция с Windows",
    settingsWinSubtitle: "Проводник и поведение утилиты",
    settingsContextMenu: "Контекстное меню Проводника",
    settingsContextMenuSub: "Пункт «Разблокировать в FreeIt» для файлов, папок и дисков",
    settingsAutostart: "Автозапуск с Windows",
    settingsAutostartSub: "Фоновый запуск FreeIt в системном трее при старте системы",
    settingsSafeVault: "Safe Vault (Теневой бэкап)",
    settingsSafeVaultSub: "Резервирование копии файла перед принудительным снятием дескрипторов",
    settingsTrayStyle: "Стиль значка в системном трее",
    settingsTrayWhite: "Белый монохром (Нативный стиль Windows 10/11)",
    settingsTrayCyan: "Фирменный циан (#00BCD4 Kobalt Tools)",
    settingsSaveBtn: "Сохранить настройки",
    settingsBackBtn: "Вернуться к инспектору файлов",
    settingsSavedOk: "Настройки успешно сохранены!",
  },
  en: {
    appName: "FreeIt",
    tabInspector: "File & Process Inspector",
    tabSettings: "Application Settings & AI Config",
    btnMinimize: "Minimize window",
    btnCloseTray: "Minimize to tray (Esc)",

    dropZoneDrag: "Drop file or folder to unlock!",
    dropZoneDragSub: "Win32 Restart Manager will intercept handles",
    dropZoneIdle: "Drag and drop locked file or folder here",
    dropZoneIdleSub: "Window stays open on blur • Windows Explorer context menu",
    pathPlaceholder: "C:\\Path\\to\\locked\\file\\or\\folder...",
    btnPickFile: "Browse file in Windows Explorer",
    btnPickFolder: "Browse folder in Windows Explorer",
    btnPasteClipboard: "Paste path from clipboard",
    btnInspect: "Check file or folder locks (Enter)",

    statusWaiting: "Pending",
    statusLocked: "🔒 Locked",
    statusFree: "🟢 Free",
    statusIdleMsg: "Drag a file here, enter a path, or use browse buttons",
    statusInspectingMsg: "Analyzing Win32 Restart Manager handles...",
    statusFreeMsg: "File is free and not locked by any process",
    statusLockedMsg: "Detected processes holding file/folder: {count}",
    statusErrorMsg: "Analysis error: {err}",

    actionUnlockAll: "Unlock: terminate all holding processes ({count})",
    actionUnlockAllDisabled: "File is free, unlocking is not needed",
    actionRename: "Rename file or folder (with automatic unlocking)",
    actionDeleteTrash: "Move to Windows Recycle Bin",
    actionDeleteTrashLocked: "Unlock and move to Windows Recycle Bin",
    actionAiExplainer: "Expert Gemini AI analysis: file purpose, lock reason, and advice",

    renamePlaceholder: "New file or folder name...",
    renameApply: "Apply",
    renameCancel: "Cancel",
    renameSuccess: "Successfully renamed to \"{name}\"",
    renameError: "Rename error: {err}",

    confirmDeleteTrash: "Move \"{path}\" to Recycle Bin?",
    confirmDeleteTrashLocked: "Object is locked by processes ({count}). Unlock and move \"{path}\" to Recycle Bin?",
    confirmKillDanger: "WARNING: \"{name}\" is a critical Windows system process! Terminating it may cause system crash or unexpected reboot. Are you absolutely sure?",

    processTitle: "Locking Processes",
    processCount: "Holding handle: {count}",
    processPid: "PID: {pid}",
    processRiskSafe: "Safe",
    processRiskWarning: "Warning",
    processRiskDanger: "Danger",
    processRiskSafeDesc: "Standard application. Safe to terminate for Windows.",
    processRiskWarningDesc: "User application. Open unsaved work may be lost.",
    processRiskDangerDesc: "Critical Windows system component. Termination is prohibited or extremely risky!",
    processServiceBadge: "Service",
    processKillBtn: "Terminate process (Unlock)",
    processKillSuccess: "Process PID {pid} successfully terminated",
    processKillError: "Failed to terminate PID {pid}: {err}",

    aiTitle: "Gemini AI Expert Analysis",
    aiAnalyzing: "AI is analyzing handles, PE metadata, and file context...",
    aiFilePurpose: "File Purpose:",
    aiLockReason: "Lock Reason:",
    aiRecommendation: "Recommendation:",
    aiSafeVerdict: "Safe for Windows system",
    aiUnsafeVerdict: "Warning: System file or risk of data loss",

    settingsTitle: "FreeIt Settings",
    settingsAppearance: "Appearance & Language",
    settingsLanguage: "Interface Language",
    settingsLanguageRu: "Русский (RU)",
    settingsLanguageEn: "English (EN)",
    settingsTheme: "Color Theme",
    settingsThemeDark: "Dark Acrylic",
    settingsThemeLight: "Light Acrylic",
    settingsThemeSystem: "Windows System",
    settingsAiSection: "Gemini AI Intelligence",
    settingsAiSubtitle: "Kobalt Tools Ecosystem • Shared Registry Key with PolyShift",
    settingsApiKey: "Google Gemini API Key",
    settingsApiKeyPlaceholder: "AIzaSy...",
    settingsBtnTestKey: "Test Google AI API connection and measure latency (ping)",
    settingsKeySharedHint: "Key is stored in HKCU\\Software\\KobaltTools\\API",
    settingsAiStudioLink: "Get API Key in Google AI Studio ↗",
    settingsModel: "AI Model (Loaded from API)",
    settingsModelRefresh: "Refresh models list from Google API",
    settingsModelLoading: "Fetching available models from Google API...",
    settingsModelsCount: "Available models in your account: {count}",
    settingsModelsEmpty: "Enter an API key above and click \"⚡ Check\" to load available models directly from Google API.",
    settingsWinSection: "Windows Integration",
    settingsWinSubtitle: "Explorer context menu & startup",
    settingsContextMenu: "Explorer Context Menu",
    settingsContextMenuSub: "\"Unlock with FreeIt\" item for files, folders, and drives",
    settingsAutostart: "Start with Windows",
    settingsAutostartSub: "Launch FreeIt minimized to system tray on Windows startup",
    settingsSafeVault: "Safe Vault (Shadow Backup)",
    settingsSafeVaultSub: "Create backup copy before forceful handle termination",
    settingsTrayStyle: "Tray Icon Style",
    settingsTrayWhite: "Monochrome White (Native Windows 10/11)",
    settingsTrayCyan: "Signature Cyan (#00BCD4 Kobalt Tools)",
    settingsSaveBtn: "Save Settings",
    settingsBackBtn: "Back to File Inspector",
    settingsSavedOk: "Settings saved successfully!",
  },
};
