import { useContext, type HTMLProps } from "react";
import { useTranslation } from "react-i18next";

import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";

import { ControlButton } from "./ControlButton";
import ControlsContext from "./ControlsContext";
import { ControlsIcons } from "./icons";

// FIXME: fix opacity bg-[rgba(var(--moss-windows-close-button-background))]/90
// FIXME: analyze and fix dark:...

export function WindowsControls({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const { isWindowMaximized, minimizeWindow, maximizeWindow, closeWindow } = useContext(ControlsContext);
  const { showDebugPanels, setShowDebugPanels } = useTabbedPaneStore();
  const { t } = useTranslation(["ns1", "ns2"]);

  const onAddPanel = (type?: string) => {
    const api = useTabbedPaneStore.getState().api;
    if (type && api?.getPanel(type) !== undefined) {
      api.getPanel(type)?.focus();
      return;
    }

    api?.addPanel({
      id: type && type !== "nested" ? type : `id_${Date.now().toString()}`,
      component: type ?? "Default",
      title: type ?? `Tab ${Date.now()}`,
      renderer: "onlyWhenVisible",
    });
  };

  return (
    <div className={cn("flex h-full", className)} {...props}>
      <ControlButton
        onClick={() => onAddPanel("Home")}
        className="active:background-(--moss-button-icon-color)/[.03] h-full w-[46px] cursor-default rounded-none bg-transparent text-(--moss-button-icon-color)/90 hover:bg-[#0000000d]"
        title={t("home")}
      >
        <span className="material-symbols-outlined">home</span>
      </ControlButton>
      <ControlButton
        onClick={() => onAddPanel("Settings")}
        className="active:background-(--moss-button-icon-color)/[.03] h-full w-[46px] cursor-default rounded-none bg-transparent text-(--moss-button-icon-color)/90 hover:bg-[#0000000d]"
        title={t("settings")}
      >
        <span className="material-symbols-outlined">settings</span>
      </ControlButton>
      <ControlButton
        onClick={() => onAddPanel("Logs")}
        className="active:background-(--moss-button-icon-color)/[.03] h-full w-[46px] cursor-default rounded-none bg-transparent text-(--moss-button-icon-color)/90 hover:bg-[#0000000d]"
        title={t("logs")}
      >
        <span className="material-symbols-outlined">terminal</span>
      </ControlButton>
      <ControlButton
        onClick={() => setShowDebugPanels(!showDebugPanels)}
        className="active:background-(--moss-button-icon-color)/[.03] h-full w-[46px] cursor-default rounded-none bg-transparent text-(--moss-button-icon-color)/90 hover:bg-[#0000000d]"
      >
        <span className="material-symbols-outlined">{showDebugPanels ? "hide_source" : "pest_control"}</span>
      </ControlButton>
      <ControlButton
        onClick={minimizeWindow}
        className="active:background-(--moss-button-icon-color)/[.03] h-full w-[46px] cursor-default rounded-none bg-transparent text-(--moss-button-icon-color)/90 hover:bg-[#0000000d]"
      >
        <ControlsIcons.minimizeWin />
      </ControlButton>
      <ControlButton
        onClick={maximizeWindow}
        className={cn(
          "h-full w-[46px] cursor-default rounded-none bg-transparent",
          "active:background-(--moss-button-icon-color)/[.03] text-(--moss-button-icon-color)/90 hover:bg-[#0000000d]"
        )}
      >
        {isWindowMaximized ? <ControlsIcons.maximizeRestoreWin /> : <ControlsIcons.maximizeWin />}
      </ControlButton>
      <ControlButton
        onClick={closeWindow}
        className="active:background-(--moss-windowsCloseButton-background)/90 hover:background-(--moss-windowsCloseButton-background) h-full w-[46px] cursor-default rounded-none bg-transparent text-(--moss-button-icon-color)/90 hover:text-white"
      >
        <ControlsIcons.closeWin />
      </ControlButton>
    </div>
  );
}
