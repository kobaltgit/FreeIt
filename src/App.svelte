<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { getCurrentWebview } from "@tauri-apps/api/webview";
  import Settings, { type AppSettings } from "./lib/Settings.svelte";
  import { translations, type Language } from "./lib/i18n";

  interface ProcessInfo {
    pid: number;
    name: string;
    app_name: string;
    friendly_name?: string;
    exe_path?: string;
    window_title?: string;
    risk_level: "safe" | "warning" | "danger";
    is_service: boolean;
  }

  interface AiExplanation {
    file_purpose: string;
    lock_reason: string;
    recommendation: string;
    is_safe: boolean;
  }

  // Svelte 5 Runes: $state
  let activeTab = $state<"inspect" | "settings">("inspect");
  let targetPath = $state("");
  let isInspecting = $state(false);
  let lockingProcesses = $state<ProcessInfo[]>([]);
  let hasChecked = $state(false);
  let statusMessage = $state("");
  let isDragging = $state(false);
  let terminatingPid = $state<number | null>(null);

  // Язык и тема интерфейса
  let currentLang = $state<Language>("ru");
  let currentTheme = $state<"dark" | "light" | "system">("dark");
  let t = $derived(translations[currentLang] || translations.ru);

  // Состояние ИИ-консультанта и файловых операций
  let aiExplanation = $state<AiExplanation | null>(null);
  let isExplaining = $state(false);
  let aiError = $state<string | null>(null);
  let isDeleting = $state(false);
  let isUnlocking = $state(false);
  let isRenaming = $state(false);
  let showRenameInput = $state(false);
  let newFileName = $state("");

  // Svelte 5 Runes: $derived
  let lockCount = $derived(lockingProcesses.length);
  let statusBadge = $derived(
    !hasChecked
      ? t.statusWaiting
      : lockCount > 0
      ? `${t.statusLocked} (${lockCount})`
      : t.statusFree
  );

  let displayStatusMessage = $derived(
    statusMessage || t.statusIdleMsg
  );

  onMount(() => {
    let unlistenDragDrop: UnlistenFn | null = null;
    let unlistenTab: UnlistenFn | null = null;
    let unlistenInspectPath: UnlistenFn | null = null;

    loadAppSettings();

    // 1. Нативная подписка на Drag & Drop Tauri v2
    getCurrentWebview()
      .onDragDropEvent((event) => {
        if (event.payload.type === "enter" || event.payload.type === "over") {
          isDragging = true;
        } else if (event.payload.type === "drop") {
          isDragging = false;
          if (event.payload.paths && event.payload.paths.length > 0) {
            activeTab = "inspect";
            targetPath = event.payload.paths[0];
            inspectPath();
          }
        } else if (event.payload.type === "leave") {
          isDragging = false;
        }
      })
      .then((unlisten) => {
        unlistenDragDrop = unlisten;
      })
      .catch((err) => console.error("DragDrop listener error:", err));

    // 2. Слушатель вызова из контекстного меню Проводника Windows (Single-Instance IPC)
    listen<string>("inspect-path", (e) => {
      if (e.payload) {
        activeTab = "inspect";
        targetPath = e.payload;
        inspectPath();
      }
    })
      .then((unlisten) => {
        unlistenInspectPath = unlisten;
      })
      .catch((err) => console.error("Inspect-path listener error:", err));

    // 3. Слушатель переключения вкладок из системного трея
    listen<string>("switch-tab", (e) => {
      if (e.payload === "settings" || e.payload === "inspect") {
        activeTab = e.payload;
      }
    })
      .then((unlisten) => {
        unlistenTab = unlisten;
      })
      .catch((err) => console.error("Switch-tab listener error:", err));

    // 4. Скрытие окна по нажатию Escape
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        hideToTray();
      }
    };
    window.addEventListener("keydown", onKeyDown);

    return () => {
      if (unlistenDragDrop) unlistenDragDrop();
      if (unlistenInspectPath) unlistenInspectPath();
      if (unlistenTab) unlistenTab();
      window.removeEventListener("keydown", onKeyDown);
    };
  });

  async function loadAppSettings() {
    try {
      const data = await invoke<AppSettings>("get_settings");
      if (data.language === "en" || data.language === "ru") {
        currentLang = data.language;
      }
      if (data.theme) {
        currentTheme = data.theme;
        applyTheme(data.theme);
      }
    } catch (err) {
      console.error("Failed to load initial settings:", err);
    }
  }

  function applyTheme(theme: "dark" | "light" | "system") {
    const isDark =
      theme === "system"
        ? window.matchMedia("(prefers-color-scheme: dark)").matches
        : theme === "dark";
    document.documentElement.setAttribute("data-theme", isDark ? "dark" : "light");
  }

  async function toggleLanguage() {
    currentLang = currentLang === "ru" ? "en" : "ru";
    try {
      const current = await invoke<AppSettings>("get_settings");
      current.language = currentLang;
      await invoke("save_settings", { settings: current });
    } catch (e) {
      console.error("Failed to save language toggle:", e);
    }
  }

  function handleSettingsChanged(s: AppSettings) {
    if (s.language === "ru" || s.language === "en") {
      currentLang = s.language;
    }
    if (s.theme) {
      currentTheme = s.theme;
      applyTheme(s.theme);
    }
  }

  async function inspectPath() {
    const path = targetPath.trim();
    if (!path) {
      statusMessage = t.statusIdleMsg;
      return;
    }

    isInspecting = true;
    aiExplanation = null;
    aiError = null;
    statusMessage = t.statusInspectingMsg;
    try {
      const results = await invoke<ProcessInfo[]>("get_locking_processes", { path });
      lockingProcesses = results;
      hasChecked = true;
      if (results.length === 0) {
        statusMessage = t.statusFreeMsg;
      } else {
        statusMessage = t.statusLockedMsg.replace("{count}", results.length.toString());
      }
    } catch (err) {
      statusMessage = t.statusErrorMsg.replace("{err}", String(err));
      lockingProcesses = [];
    } finally {
      isInspecting = false;
    }
  }

  async function killProcess(pid: number) {
    const proc = lockingProcesses.find((p) => p.pid === pid);
    if (proc && proc.risk_level === "danger") {
      const warningMsg = t.confirmKillDanger.replace("{name}", proc.friendly_name || proc.name);
      if (!confirm(warningMsg)) return;
    }

    terminatingPid = pid;
    try {
      await invoke("kill_process", { pid });
      statusMessage = t.processKillSuccess.replace("{pid}", pid.toString());
      await inspectPath();
    } catch (err) {
      statusMessage = t.processKillError.replace("{pid}", pid.toString()).replace("{err}", String(err));
    } finally {
      terminatingPid = null;
    }
  }

  async function requestAiExplanation() {
    const path = targetPath.trim();
    if (!path) return;

    isExplaining = true;
    aiError = null;
    aiExplanation = null;
    try {
      const result = await invoke<AiExplanation>("explain_target", {
        path,
        processes: lockingProcesses,
        lang: currentLang,
      });
      aiExplanation = result;
    } catch (err: any) {
      aiError = typeof err === "string" ? err : err?.message || (currentLang === "en" ? "Failed to obtain Gemini AI analysis" : "Не удалось получить ответ Gemini AI");
    } finally {
      isExplaining = false;
    }
  }

  async function handleUnlockAll() {
    if (lockCount === 0) return;
    const pids = lockingProcesses.filter((p) => p.risk_level !== "danger").map((p) => p.pid);
    if (pids.length === 0) {
      statusMessage = currentLang === "en"
        ? "Object is held only by critical Windows system processes"
        : "Объект удерживается только критическими системными процессами Windows";
      return;
    }
    isUnlocking = true;
    try {
      const killed = await invoke<number>("unlock_all", { pids });
      statusMessage = currentLang === "en"
        ? `Successfully terminated processes: ${killed}`
        : `Успешно завершено процессов: ${killed}`;
      await inspectPath();
    } catch (err: any) {
      statusMessage = (currentLang === "en" ? "Unlock error: " : "Ошибка разблокировки: ") + err;
    } finally {
      isUnlocking = false;
    }
  }

  function toggleRename() {
    if (showRenameInput) {
      showRenameInput = false;
      newFileName = "";
    } else {
      const clean = targetPath.replace(/[\\/]+$/, "");
      const parts = clean.split(/[\\/]/);
      newFileName = parts[parts.length - 1] || "";
      showRenameInput = true;
    }
  }

  function cancelRename() {
    showRenameInput = false;
    newFileName = "";
  }

  async function applyRename() {
    const trimmed = newFileName.trim();
    if (!trimmed) return;
    isRenaming = true;
    try {
      if (lockCount > 0) {
        const pids = lockingProcesses.filter((p) => p.risk_level !== "danger").map((p) => p.pid);
        if (pids.length > 0) {
          await invoke("unlock_all", { pids });
        }
      }
      const updatedPath = await invoke<string>("rename_target", {
        oldPath: targetPath,
        newName: trimmed,
      });
      targetPath = updatedPath;
      showRenameInput = false;
      statusMessage = t.renameSuccess.replace("{name}", trimmed);
      await inspectPath();
    } catch (err: any) {
      statusMessage = t.renameError.replace("{err}", String(err));
    } finally {
      isRenaming = false;
    }
  }

  async function handleDeleteToTrash() {
    const path = targetPath.trim();
    if (!path) return;

    const confirmMsg =
      lockCount > 0
        ? t.confirmDeleteTrashLocked.replace("{count}", lockCount.toString()).replace("{path}", path)
        : t.confirmDeleteTrash.replace("{path}", path);

    if (!confirm(confirmMsg)) return;

    isDeleting = true;
    try {
      if (lockCount > 0) {
        const pids = lockingProcesses.filter((p) => p.risk_level !== "danger").map((p) => p.pid);
        if (pids.length > 0) {
          await invoke("unlock_all", { pids });
        }
      }

      await invoke("delete_to_trash", { path });
      statusMessage = currentLang === "en" ? "Object moved to Windows Recycle Bin" : "Объект успешно перемещен в Корзину Windows";
      hasChecked = false;
      lockingProcesses = [];
      aiExplanation = null;
      showRenameInput = false;
      targetPath = "";
    } catch (err: any) {
      statusMessage = (currentLang === "en" ? "Delete error: " : "Ошибка удаления: ") + err;
    } finally {
      isDeleting = false;
    }
  }

  async function pickFile() {
    try {
      const path = await invoke<string | null>("pick_file_dialog");
      if (path) {
        targetPath = path;
        activeTab = "inspect";
        inspectPath();
      }
    } catch (err) {
      console.error("pick_file error:", err);
    }
  }

  async function pickFolder() {
    try {
      const path = await invoke<string | null>("pick_folder_dialog");
      if (path) {
        targetPath = path;
        activeTab = "inspect";
        inspectPath();
      }
    } catch (err) {
      console.error("pick_folder error:", err);
    }
  }

  async function pasteFromClipboard() {
    try {
      const text = await navigator.clipboard.readText();
      if (text && text.trim()) {
        targetPath = text.trim();
        inspectPath();
      }
    } catch (err) {
      console.warn("Clipboard paste error:", err);
    }
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === "Enter") {
      inspectPath();
    }
  }

  function minimizeWindow() {
    invoke("minimize_window");
  }

  function hideToTray() {
    invoke("hide_window");
  }
</script>

<main class="flyout-container" class:dragging={isDragging}>
  <!-- Верхний заголовок Fluent Acrylic с поддержкой перемещения окна (data-tauri-drag-region) -->
  <header class="flyout-header" data-tauri-drag-region>
    <div class="brand" data-tauri-drag-region>
      <div
        class="status-dot"
        class:locked={lockCount > 0}
        class:inspecting={isInspecting}
      ></div>
      <h1 data-tauri-drag-region>FreeIt</h1>
      <span class="version-tag">v1.0.0</span>
    </div>

    <!-- Вкладки и элементы управления окном (крупные иконки-обводки без текста) -->
    <div class="header-actions">
      <!-- Быстрый переключатель языка (RU / EN) -->
      <button
        class="nav-tab-btn lang-toggle-btn"
        title={currentLang === "ru" ? "Switch language to English" : "Переключить интерфейс на русский"}
        onclick={toggleLanguage}
      >
        <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"></circle>
          <line x1="2" y1="12" x2="22" y2="12"></line>
          <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path>
        </svg>
        <span class="lang-text">{currentLang.toUpperCase()}</span>
      </button>

      <button
        class="nav-tab-btn"
        class:active={activeTab === "inspect"}
        title={t.tabInspector}
        onclick={() => (activeTab = "inspect")}
      >
        <svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8"></circle>
          <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </svg>
      </button>

      <button
        class="nav-tab-btn"
        class:active={activeTab === "settings"}
        title={t.tabSettings}
        onclick={() => (activeTab = "settings")}
      >
        <svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"></circle>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
        </svg>
      </button>

      <div class="window-controls-divider"></div>

      <button class="win-btn minimize-btn" title={t.btnMinimize} onclick={minimizeWindow}>
        <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
      </button>
      <button class="win-btn close-btn" title={t.btnCloseTray} onclick={hideToTray}>
        <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
    </div>
  </header>

  <!-- Тело окна: Вкладка Инспектор или Настройки -->
  <section class="content-body">
    {#if activeTab === "settings"}
      <Settings
        onBack={() => (activeTab = "inspect")}
        onSettingsChanged={handleSettingsChanged}
      />
    {:else}
      <!-- Интерактивная Drop-Zone -->
      <div class="drop-zone" class:active-drag={isDragging}>
        <div class="drop-icon">
          <svg viewBox="0 0 24 24" width="40" height="40" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
            <line x1="12" y1="11" x2="12" y2="17"></line>
            <polyline points="9 14 12 17 15 14"></polyline>
          </svg>
        </div>
        <div class="drop-text">
          {#if isDragging}
            <span class="drop-title active-color">{t.dropZoneDrag}</span>
            <span class="drop-sub">{t.dropZoneDragSub}</span>
          {:else}
            <span class="drop-title">{t.dropZoneIdle}</span>
            <span class="drop-sub">{t.dropZoneIdleSub}</span>
          {/if}
        </div>
      </div>

      <!-- Строка ввода пути и кнопки выбора (крупные иконки-обводки с тултипами) -->
      <div class="input-row">
        <input
          type="text"
          placeholder={t.pathPlaceholder}
          bind:value={targetPath}
          onkeydown={handleKeyDown}
          disabled={isInspecting}
          class="path-input"
        />

        <button
          class="picker-btn"
          title={t.btnPickFile}
          onclick={pickFile}
          disabled={isInspecting}
        >
          <svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
            <polyline points="14 2 14 8 20 8"></polyline>
            <line x1="16" y1="13" x2="8" y2="13"></line>
            <line x1="16" y1="17" x2="8" y2="17"></line>
            <polyline points="10 9 9 9 8 9"></polyline>
          </svg>
        </button>

        <button
          class="picker-btn"
          title={t.btnPickFolder}
          onclick={pickFolder}
          disabled={isInspecting}
        >
          <svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
          </svg>
        </button>

        <button
          class="icon-btn"
          title={t.btnPasteClipboard}
          onclick={pasteFromClipboard}
          disabled={isInspecting}
        >
          <svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path>
            <rect x="8" y="2" width="8" height="4" rx="1" ry="1"></rect>
          </svg>
        </button>

        <button
          class="action-btn"
          title={t.btnInspect}
          onclick={inspectPath}
          disabled={isInspecting || !targetPath.trim()}
        >
          {#if isInspecting}
            <div class="btn-spinner"></div>
          {:else}
            <svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="11" cy="11" r="8"></circle>
              <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
              <path d="M11 8v6"></path>
              <path d="M8 11h6"></path>
            </svg>
          {/if}
        </button>
      </div>

      <!-- Статус-бар и быстрые действия -->
      <div class="status-panel">
        <div class="status-banner">
          <span class="badge" class:badge-locked={lockCount > 0} class:badge-free={hasChecked && lockCount === 0}>
            {statusBadge}
          </span>
          <span class="status-text">{displayStatusMessage}</span>
        </div>

        {#if hasChecked && targetPath.trim()}
          <div class="quick-actions-bar">
            <!-- 1. Разблокировать всё -->
            <button
              class="unlock-action-btn"
              disabled={lockCount === 0 || isUnlocking}
              onclick={handleUnlockAll}
              title={lockCount > 0 ? t.actionUnlockAll.replace("{count}", lockCount.toString()) : t.actionUnlockAllDisabled}
            >
              {#if isUnlocking}
                <div class="btn-spinner"></div>
              {:else}
                <svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                  <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
                  <path d="M7 11V7a5 5 0 0 1 9.9-1"></path>
                </svg>
              {/if}
            </button>

            <!-- 2. Переименовать -->
            <button
              class="rename-action-btn"
              class:active={showRenameInput}
              disabled={isRenaming}
              onclick={toggleRename}
              title={t.actionRename}
            >
              <svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
                <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
              </svg>
            </button>

            <!-- 3. Удалить в Корзину (ВСЕГДА ВИДНА) -->
            <button
              class="trash-action-btn"
              disabled={isDeleting}
              onclick={handleDeleteToTrash}
              title={lockCount > 0 ? t.actionDeleteTrashLocked : t.actionDeleteTrash}
            >
              {#if isDeleting}
                <div class="btn-spinner"></div>
              {:else}
                <svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                  <polyline points="3 6 5 6 21 6"></polyline>
                  <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
                  <line x1="10" y1="11" x2="10" y2="17"></line>
                  <line x1="14" y1="11" x2="14" y2="17"></line>
                </svg>
              {/if}
            </button>

            <!-- 4. ИИ-разбор Gemini -->
            <button
              class="ai-action-btn"
              disabled={isExplaining}
              onclick={requestAiExplanation}
              title={t.actionAiExplainer}
            >
              {#if isExplaining}
                <div class="btn-spinner"></div>
              {:else}
                <svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                  <path d="m12 3-1.9 5.8a2 2 0 0 1-1.3 1.3L3 12l5.8 1.9a2 2 0 0 1 1.3 1.3L12 21l1.9-5.8a2 2 0 0 1 1.3-1.3L21 12l-5.8-1.9a2 2 0 0 1-1.3-1.3z"></path>
                </svg>
              {/if}
            </button>
          </div>
        {/if}

        <!-- Инлайн-панель переименования -->
        {#if showRenameInput}
          <div class="rename-panel">
            <input
              type="text"
              bind:value={newFileName}
              placeholder={t.renamePlaceholder}
              class="rename-input"
              disabled={isRenaming}
              onkeydown={(e) => {
                if (e.key === 'Enter') applyRename();
                else if (e.key === 'Escape') cancelRename();
              }}
            />
            <button
              class="rename-btn apply-btn"
              onclick={applyRename}
              disabled={isRenaming || !newFileName.trim()}
              title={t.renameApply}
            >
              {#if isRenaming}
                <div class="btn-spinner"></div>
              {:else}
                <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <polyline points="20 6 9 17 4 12"></polyline>
                </svg>
              {/if}
            </button>
            <button
              class="rename-btn cancel-btn"
              onclick={cancelRename}
              disabled={isRenaming}
              title={t.renameCancel}
            >
              <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18"></line>
                <line x1="6" y1="6" x2="18" y2="18"></line>
              </svg>
            </button>
          </div>
        {/if}
      </div>

      <!-- Карточка экспертного разбора Gemini AI -->
      {#if aiExplanation}
        <div class="ai-insight-card">
          <div class="ai-card-header">
            <div class="ai-card-title">
              <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="#c084fc" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <path d="m12 3-1.9 5.8a2 2 0 0 1-1.3 1.3L3 12l5.8 1.9a2 2 0 0 1 1.3 1.3L12 21l1.9-5.8a2 2 0 0 1 1.3-1.3L21 12l-5.8-1.9a2 2 0 0 1-1.3-1.3z"></path>
              </svg>
              <strong>{t.aiTitle}</strong>
            </div>
            <button class="ai-close-btn" onclick={() => (aiExplanation = null)} title="Close">
              ✕
            </button>
          </div>

          <div class="ai-insight-content">
            <div class="ai-insight-row">
              <div class="ai-row-label">📄 {t.aiFilePurpose}</div>
              <div class="ai-row-val">{aiExplanation.file_purpose}</div>
            </div>

            <div class="ai-insight-row">
              <div class="ai-row-label">🔒 {t.aiLockReason}</div>
              <div class="ai-row-val">{aiExplanation.lock_reason}</div>
            </div>

            <div class="ai-insight-row">
              <div class="ai-row-label">💡 {t.aiRecommendation}</div>
              <div class="ai-row-val recommendation">{aiExplanation.recommendation}</div>
            </div>
          </div>

          <div class="ai-card-footer">
            <span class="ai-safety-badge" class:safe={aiExplanation.is_safe} class:unsafe={!aiExplanation.is_safe}>
              {aiExplanation.is_safe ? t.aiSafeVerdict : t.aiUnsafeVerdict}
            </span>
          </div>
        </div>
      {:else if aiError}
        <div class="ai-error-banner">
          <span>⚠️ {aiError}</span>
          <button class="goto-settings-btn" title={t.tabSettings} onclick={() => (activeTab = "settings")}>
            ⚙️ {t.tabSettings}
          </button>
        </div>
      {/if}

      <!-- Результаты: список процессов -->
      <div class="results-container">
        {#if lockingProcesses.length > 0}
          <div class="processes-header">
            <span>{t.processTitle} ({lockingProcesses.length}):</span>
          </div>

          <div class="process-list">
            {#each lockingProcesses as proc}
              <div class="process-card risk-{proc.risk_level}">
                <div class="process-top">
                  <div class="proc-name-group">
                    <span class="risk-dot risk-dot-{proc.risk_level}"></span>
                    <span class="proc-name">
                      {proc.friendly_name || proc.name}
                    </span>
                  </div>
                  <span class="pid-tag">{t.processPid.replace("{pid}", proc.pid.toString())}</span>
                </div>

                {#if proc.friendly_name && proc.name !== proc.friendly_name}
                  <div class="proc-sub">{currentLang === "en" ? "File name:" : "Имя файла:"} <code>{proc.name}</code></div>
                {/if}

                {#if proc.window_title}
                  <div class="proc-sub title-sub">
                    {currentLang === "en" ? "Window title:" : "Заголовок окна:"} <em>«{proc.window_title}»</em>
                  </div>
                {/if}

                {#if proc.exe_path}
                  <div class="proc-path" title={proc.exe_path}>
                    {proc.exe_path}
                  </div>
                {/if}

                <div class="card-footer">
                  <span class="risk-badge risk-badge-{proc.risk_level}">
                    {#if proc.risk_level === 'safe'}
                      🟢 {t.processRiskSafe}
                    {:else if proc.risk_level === 'warning'}
                      🟡 {t.processRiskWarning}
                    {:else}
                      🔴 {t.processRiskDanger}
                    {/if}
                  </span>

                  <button
                    class="kill-btn"
                    disabled={terminatingPid === proc.pid || proc.risk_level === 'danger'}
                    onclick={() => killProcess(proc.pid)}
                    title={proc.risk_level === 'danger' ? t.processRiskDangerDesc : t.processKillBtn}
                  >
                    {#if terminatingPid === proc.pid}
                      <div class="btn-spinner"></div>
                    {:else}
                      <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="12" cy="12" r="10"></circle>
                        <line x1="15" y1="9" x2="9" y2="15"></line>
                        <line x1="9" y1="9" x2="15" y2="15"></line>
                      </svg>
                    {/if}
                  </button>
                </div>
              </div>
            {/each}
          </div>
        {:else if hasChecked && !isInspecting}
          <div class="empty-card">
            <div class="check-icon">✓</div>
            <p>{t.statusFreeMsg}</p>
            <span class="subtext">{currentLang === "en" ? "File handles are free, object can be safely moved or removed" : "Дескрипторы не удерживаются процессами, объект можно безопасно перемещать или удалять"}</span>
          </div>
        {/if}
      </div>
    {/if}
  </section>

  <!-- Футер в стиле Kobalt Tools -->
  <footer class="flyout-footer" data-tauri-drag-region>
    <span class="version">Kobalt FreeIt • Win32 Restart Manager • {currentLang === "en" ? "Recycle Bin & MiniBin Sync" : "Синхронизация с Корзиной и MiniBin"}</span>
  </footer>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: "Segoe UI Variable Text", "Segoe UI", -apple-system, sans-serif;
    user-select: none;
    background: transparent;
    overflow: hidden;
  }

  .flyout-container {
    width: 100vw;
    height: 100vh;
    padding: 12px 16px;
    box-sizing: border-box;
    background: rgba(18, 24, 38, 0.95);
    backdrop-filter: blur(28px) saturate(180%);
    -webkit-backdrop-filter: blur(28px) saturate(180%);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 10px;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.7);
    color: #ffffff;
    display: flex;
    flex-direction: column;
    transition: border-color 0.2s ease, background 0.2s ease;
  }

  .flyout-container.dragging {
    border-color: #00bcd4;
    background: rgba(18, 30, 48, 0.98);
  }

  .flyout-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
    padding-bottom: 8px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    cursor: default;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: default;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #10b981;
    box-shadow: 0 0 8px #10b981;
    transition: all 0.2s ease;
  }

  .status-dot.locked {
    background: #ef4444;
    box-shadow: 0 0 8px #ef4444;
  }

  .status-dot.inspecting {
    background: #00bcd4;
    box-shadow: 0 0 8px #00bcd4;
  }

  h1 {
    font-size: 15px;
    font-weight: 600;
    margin: 0;
    letter-spacing: 0.3px;
    color: #ffffff;
  }

  .version-tag {
    font-size: 10px;
    color: #64748b;
    background: rgba(255, 255, 255, 0.05);
    padding: 1px 5px;
    border-radius: 4px;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .nav-tab-btn {
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    border-radius: 6px;
    padding: 4px 10px;
    font-size: 12px;
    font-weight: 500;
    display: flex;
    align-items: center;
    gap: 4px;
    transition: all 0.15s ease;
  }

  .nav-tab-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    color: #ffffff;
  }

  .nav-tab-btn.active {
    background: rgba(0, 188, 212, 0.18);
    color: #00bcd4;
    font-weight: 600;
  }

  .window-controls-divider {
    width: 1px;
    height: 16px;
    background: rgba(255, 255, 255, 0.12);
    margin: 0 4px;
  }

  .win-btn {
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    border-radius: 4px;
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    transition: all 0.15s ease;
  }

  .win-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #ffffff;
  }

  .win-btn.close-btn:hover {
    background: #ef4444;
    color: #ffffff;
  }

  .content-body {
    display: flex;
    flex-direction: column;
    flex: 1;
    overflow: hidden;
    gap: 10px;
  }

  .drop-zone {
    border: 1.5px dashed rgba(255, 255, 255, 0.18);
    border-radius: 8px;
    padding: 12px 14px;
    display: flex;
    align-items: center;
    gap: 12px;
    background: rgba(255, 255, 255, 0.02);
    transition: all 0.2s ease;
  }

  .drop-zone.active-drag {
    border-color: #00bcd4;
    background: rgba(0, 188, 212, 0.08);
    box-shadow: inset 0 0 14px rgba(0, 188, 212, 0.2);
  }

  .drop-icon {
    font-size: 26px;
    opacity: 0.85;
  }

  .drop-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .drop-title {
    font-size: 13px;
    font-weight: 500;
    color: #e2e8f0;
  }

  .drop-title.active-color {
    color: #00bcd4;
    font-weight: 600;
  }

  .drop-sub {
    font-size: 11px;
    color: #64748b;
  }

  .input-row {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  .path-input {
    flex: 1;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.14);
    border-radius: 6px;
    padding: 7px 10px;
    font-size: 12px;
    color: #ffffff;
    outline: none;
    transition: border-color 0.15s ease;
  }

  .path-input:focus {
    border-color: #00bcd4;
    background: rgba(255, 255, 255, 0.09);
  }

  .picker-btn,
  .icon-btn {
    width: 34px;
    height: 34px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.14);
    border-radius: 6px;
    cursor: pointer;
    color: #cbd5e1;
    flex-shrink: 0;
    transition: background 0.15s ease, border-color 0.15s ease, color 0.15s ease, transform 0.1s ease;
  }

  .picker-btn:hover:not(:disabled),
  .icon-btn:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.14);
    border-color: rgba(255, 255, 255, 0.3);
    color: #ffffff;
  }

  .picker-btn:active:not(:disabled),
  .icon-btn:active:not(:disabled) {
    transform: scale(0.96);
  }

  .picker-btn:disabled,
  .icon-btn:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .action-btn {
    width: 34px;
    height: 34px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: #00bcd4;
    color: #0c1824;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    flex-shrink: 0;
    transition: filter 0.15s ease, transform 0.1s ease;
  }

  .action-btn:hover:not(:disabled) {
    filter: brightness(1.18);
  }

  .action-btn:active:not(:disabled) {
    transform: scale(0.96);
  }

  .action-btn:disabled {
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

  .status-panel {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .status-banner {
    background: rgba(255, 255, 255, 0.03);
    border-radius: 6px;
    padding: 6px 10px;
    font-size: 11px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .status-text {
    color: #94a3b8;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
  }

  .badge {
    font-size: 10px;
    padding: 2px 6px;
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.08);
    color: #94a3b8;
    flex-shrink: 0;
  }

  .badge-locked {
    background: rgba(239, 68, 68, 0.18);
    color: #fca5a5;
    border: 1px solid rgba(239, 68, 68, 0.3);
  }

  .badge-free {
    background: rgba(16, 185, 129, 0.18);
    color: #6ee7b7;
    border: 1px solid rgba(16, 185, 129, 0.3);
  }

  .quick-actions-bar {
    display: flex;
    gap: 8px;
  }

  .unlock-action-btn {
    width: 34px;
    height: 34px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: rgba(234, 179, 8, 0.16);
    border: 1px solid rgba(234, 179, 8, 0.4);
    color: #fde047;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    flex-shrink: 0;
  }

  .unlock-action-btn:hover:not(:disabled) {
    background: rgba(234, 179, 8, 0.28);
    border-color: #eab308;
    color: #ffffff;
    transform: scale(1.04);
  }

  .unlock-action-btn:active:not(:disabled) {
    transform: scale(0.96);
  }

  .unlock-action-btn:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }

  .rename-action-btn {
    width: 34px;
    height: 34px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: rgba(59, 130, 246, 0.16);
    border: 1px solid rgba(59, 130, 246, 0.4);
    color: #93c5fd;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    flex-shrink: 0;
  }

  .rename-action-btn:hover:not(:disabled) {
    background: rgba(59, 130, 246, 0.28);
    border-color: #3b82f6;
    color: #ffffff;
    transform: scale(1.04);
  }

  .rename-action-btn.active {
    background: rgba(59, 130, 246, 0.35);
    border-color: #60a5fa;
    color: #ffffff;
    box-shadow: 0 0 10px rgba(59, 130, 246, 0.3);
  }

  .rename-action-btn:active:not(:disabled) {
    transform: scale(0.96);
  }

  .rename-action-btn:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }

  .trash-action-btn {
    width: 34px;
    height: 34px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: rgba(239, 68, 68, 0.15);
    border: 1px solid rgba(239, 68, 68, 0.35);
    color: #fca5a5;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    flex-shrink: 0;
  }

  .trash-action-btn:hover:not(:disabled) {
    background: rgba(239, 68, 68, 0.28);
    border-color: #ef4444;
    color: #ffffff;
    transform: scale(1.04);
  }

  .trash-action-btn:active:not(:disabled) {
    transform: scale(0.96);
  }

  .trash-action-btn:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .ai-action-btn {
    width: 34px;
    height: 34px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, rgba(147, 51, 234, 0.25), rgba(0, 188, 212, 0.25));
    border: 1px solid rgba(0, 188, 212, 0.4);
    color: #a5f3fc;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    flex-shrink: 0;
  }

  .ai-action-btn:hover:not(:disabled) {
    background: linear-gradient(135deg, rgba(147, 51, 234, 0.4), rgba(0, 188, 212, 0.4));
    border-color: #00bcd4;
    color: #ffffff;
    transform: scale(1.04);
  }

  .ai-action-btn:active:not(:disabled) {
    transform: scale(0.96);
  }

  .ai-action-btn:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .rename-panel {
    display: flex;
    gap: 6px;
    align-items: center;
    background: rgba(15, 23, 42, 0.85);
    border: 1px solid rgba(59, 130, 246, 0.35);
    border-radius: 6px;
    padding: 6px 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    animation: fadeIn 0.2s ease;
  }

  .rename-input {
    flex: 1;
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.18);
    border-radius: 4px;
    padding: 5px 8px;
    font-size: 12px;
    color: #ffffff;
    outline: none;
    transition: border-color 0.15s ease;
  }

  .rename-input:focus {
    border-color: #3b82f6;
    background: rgba(255, 255, 255, 0.12);
  }

  .rename-btn {
    width: 28px;
    height: 28px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .rename-btn.apply-btn {
    background: #10b981;
    color: #ffffff;
  }

  .rename-btn.apply-btn:hover:not(:disabled) {
    background: #059669;
    transform: scale(1.05);
  }

  .rename-btn.cancel-btn {
    background: rgba(255, 255, 255, 0.1);
    color: #cbd5e1;
  }

  .rename-btn.cancel-btn:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.2);
    color: #ffffff;
    transform: scale(1.05);
  }

  .rename-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  /* Карточка ИИ-консультанта */
  .ai-insight-card {
    background: linear-gradient(135deg, rgba(24, 18, 43, 0.8), rgba(12, 28, 44, 0.8));
    border: 1px solid rgba(168, 85, 247, 0.3);
    border-radius: 8px;
    padding: 10px 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
    animation: fadeIn 0.25s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .ai-card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    padding-bottom: 6px;
  }

  .ai-card-title {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: #e9d5ff;
  }

  .ai-sparkle {
    font-size: 14px;
  }

  .ai-close-btn {
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.5);
    cursor: pointer;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 11px;
  }

  .ai-close-btn:hover {
    color: #fff;
    background: rgba(255, 255, 255, 0.1);
  }

  .ai-insight-content {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .ai-insight-row {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .ai-row-label {
    font-size: 11px;
    font-weight: 600;
    color: #c084fc;
  }

  .ai-row-val {
    font-size: 11.5px;
    color: #f1f5f9;
    line-height: 1.35;
  }

  .ai-row-val.recommendation {
    color: #a5f3fc;
  }

  .ai-card-footer {
    display: flex;
    justify-content: flex-start;
    padding-top: 4px;
    border-top: 1px solid rgba(255, 255, 255, 0.05);
  }

  .ai-safety-badge {
    font-size: 10px;
    padding: 2px 8px;
    border-radius: 4px;
    font-weight: 500;
  }

  .ai-safety-badge.safe {
    background: rgba(16, 185, 129, 0.15);
    color: #6ee7b7;
    border: 1px solid rgba(16, 185, 129, 0.3);
  }

  .ai-safety-badge.unsafe {
    background: rgba(239, 68, 68, 0.15);
    color: #fca5a5;
    border: 1px solid rgba(239, 68, 68, 0.3);
  }

  .ai-error-banner {
    background: rgba(239, 68, 68, 0.12);
    border: 1px solid rgba(239, 68, 68, 0.25);
    border-radius: 6px;
    padding: 8px 10px;
    font-size: 11px;
    color: #fca5a5;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 8px;
  }

  .goto-settings-btn {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.2);
    color: #fff;
    border-radius: 4px;
    padding: 2px 8px;
    font-size: 10.5px;
    cursor: pointer;
    white-space: nowrap;
  }

  .goto-settings-btn:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  /* Результаты */
  .results-container {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding-right: 4px;
  }

  .results-container::-webkit-scrollbar {
    width: 4px;
  }

  .results-container::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.15);
    border-radius: 4px;
  }

  .processes-header {
    font-size: 11px;
    text-transform: uppercase;
    color: #64748b;
    letter-spacing: 0.5px;
    margin-top: 2px;
  }

  .process-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .process-card {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    padding: 10px 12px;
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .process-card.risk-danger {
    border-left: 3px solid #ef4444;
  }

  .process-card.risk-warning {
    border-left: 3px solid #eab308;
  }

  .process-card.risk-safe {
    border-left: 3px solid #10b981;
  }

  .process-top {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .proc-name-group {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .risk-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
  }

  .risk-dot-safe {
    background: #10b981;
  }

  .risk-dot-warning {
    background: #eab308;
  }

  .risk-dot-danger {
    background: #ef4444;
  }

  .proc-name {
    font-size: 13px;
    font-weight: 600;
    color: #f1f5f9;
  }

  .pid-tag {
    font-size: 10px;
    background: rgba(255, 255, 255, 0.06);
    padding: 2px 6px;
    border-radius: 4px;
    color: #cbd5e1;
  }

  .proc-sub {
    font-size: 11px;
    color: #94a3b8;
  }

  .proc-sub code {
    color: rgba(0, 188, 212, 0.9);
    background: rgba(0, 0, 0, 0.2);
    padding: 1px 4px;
    border-radius: 3px;
  }

  .title-sub em {
    color: #e2e8f0;
    font-style: normal;
  }

  .proc-path {
    font-size: 10px;
    color: #64748b;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .card-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 4px;
    padding-top: 4px;
    border-top: 1px solid rgba(255, 255, 255, 0.05);
  }

  .risk-badge {
    font-size: 10px;
    padding: 2px 6px;
    border-radius: 4px;
  }

  .risk-badge-safe {
    background: rgba(16, 185, 129, 0.12);
    color: #6ee7b7;
  }

  .risk-badge-warning {
    background: rgba(234, 179, 8, 0.12);
    color: #fde047;
  }

  .risk-badge-danger {
    background: rgba(239, 68, 68, 0.12);
    color: #fca5a5;
  }

  .kill-btn {
    width: 32px;
    height: 32px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: rgba(239, 68, 68, 0.16);
    border: 1px solid rgba(239, 68, 68, 0.35);
    color: #fca5a5;
    border-radius: 6px;
    cursor: pointer;
    flex-shrink: 0;
    transition: background 0.15s ease, border-color 0.15s ease, color 0.15s ease, transform 0.1s ease;
  }

  .kill-btn:hover:not(:disabled) {
    background: rgba(239, 68, 68, 0.32);
    border-color: #ef4444;
    color: #ffffff;
    transform: scale(1.04);
  }

  .kill-btn:active:not(:disabled) {
    transform: scale(0.96);
  }

  .kill-btn:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }

  .empty-card {
    text-align: center;
    padding: 28px 12px;
    background: rgba(16, 185, 129, 0.05);
    border: 1px dashed rgba(16, 185, 129, 0.25);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    margin-top: 8px;
  }

  .check-icon {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: rgba(16, 185, 129, 0.15);
    color: #10b981;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 20px;
    margin-bottom: 4px;
  }

  .empty-card p {
    margin: 0;
    font-size: 14px;
    font-weight: 500;
    color: #e2e8f0;
  }

  .empty-card .subtext {
    font-size: 11px;
    color: #64748b;
  }

  .flyout-footer {
    margin-top: 8px;
    padding-top: 6px;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
    text-align: center;
    cursor: default;
  }

  .version {
    font-size: 10px;
    color: #64748b;
  }

  .lang-toggle-btn {
    padding: 3px 8px;
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.5px;
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.04);
  }

  .lang-toggle-btn:hover {
    border-color: #00bcd4;
    background: rgba(0, 188, 212, 0.12);
    color: #00bcd4;
  }

  .lang-text {
    font-size: 11px;
    font-weight: 600;
  }

  /* --- Fluent Acrylic Light Theme --- */
  :global([data-theme="light"]) .flyout-container {
    background: rgba(246, 248, 252, 0.94);
    border-color: rgba(0, 0, 0, 0.12);
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.15);
    color: #1e293b;
  }

  :global([data-theme="light"]) h1 {
    color: #0f172a;
  }

  :global([data-theme="light"]) .flyout-header {
    border-bottom-color: rgba(0, 0, 0, 0.08);
  }

  :global([data-theme="light"]) .nav-tab-btn {
    color: #64748b;
  }

  :global([data-theme="light"]) .nav-tab-btn:hover {
    background: rgba(0, 0, 0, 0.06);
    color: #0f172a;
  }

  :global([data-theme="light"]) .nav-tab-btn.active {
    background: rgba(0, 188, 212, 0.15);
    color: #0891b2;
  }

  :global([data-theme="light"]) .lang-toggle-btn {
    border-color: rgba(0, 0, 0, 0.12);
    background: rgba(0, 0, 0, 0.04);
  }

  :global([data-theme="light"]) .drop-zone {
    background: rgba(255, 255, 255, 0.65);
    border-color: rgba(0, 0, 0, 0.15);
  }

  :global([data-theme="light"]) .drop-title {
    color: #1e293b;
  }

  :global([data-theme="light"]) .drop-sub {
    color: #64748b;
  }

  :global([data-theme="light"]) .path-input {
    background: rgba(255, 255, 255, 0.85);
    border-color: rgba(0, 0, 0, 0.15);
    color: #0f172a;
  }

  :global([data-theme="light"]) .picker-btn,
  :global([data-theme="light"]) .icon-btn {
    background: rgba(255, 255, 255, 0.9);
    border-color: rgba(0, 0, 0, 0.15);
    color: #334155;
  }

  :global([data-theme="light"]) .status-panel {
    background: rgba(255, 255, 255, 0.75);
    border-color: rgba(0, 0, 0, 0.08);
  }

  :global([data-theme="light"]) .status-text {
    color: #334155;
  }

  :global([data-theme="light"]) .process-card {
    background: rgba(255, 255, 255, 0.85);
    border-color: rgba(0, 0, 0, 0.1);
  }

  :global([data-theme="light"]) .proc-name {
    color: #0f172a;
  }

  :global([data-theme="light"]) .pid-tag {
    background: rgba(0, 0, 0, 0.06);
    color: #475569;
  }

  :global([data-theme="light"]) .proc-sub {
    color: #64748b;
  }

  :global([data-theme="light"]) .proc-path {
    background: rgba(0, 0, 0, 0.04);
    color: #64748b;
  }

  :global([data-theme="light"]) .ai-insight-card {
    background: rgba(250, 245, 255, 0.9);
    border-color: rgba(192, 132, 252, 0.35);
  }

  :global([data-theme="light"]) .ai-card-title strong {
    color: #7e22ce;
  }

  :global([data-theme="light"]) .ai-row-label {
    color: #6b21a8;
  }

  :global([data-theme="light"]) .ai-row-val {
    color: #1e293b;
  }

  :global([data-theme="light"]) .rename-panel {
    background: rgba(255, 255, 255, 0.95);
  }

  :global([data-theme="light"]) .rename-input {
    background: rgba(0, 0, 0, 0.04);
    color: #0f172a;
  }

  :global([data-theme="light"]) .empty-card {
    background: rgba(255, 255, 255, 0.7);
  }

  :global([data-theme="light"]) .empty-card p {
    color: #0f172a;
  }

  :global([data-theme="light"]) .flyout-footer {
    border-top-color: rgba(0, 0, 0, 0.06);
  }
</style>