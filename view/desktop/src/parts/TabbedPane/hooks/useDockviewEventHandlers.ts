import React from "react";

import { useTabbedPaneStore } from "@/store/tabbedPane";
import { DockviewApi } from "@repo/moss-tabs";

export const useDockviewEventHandlers = (
  api: DockviewApi | undefined,
  addLogLine: (message: string) => void,
  setPanels: React.Dispatch<React.SetStateAction<string[]>>,
  setGroups: React.Dispatch<React.SetStateAction<string[]>>,
  setActivePanel: React.Dispatch<React.SetStateAction<string | undefined>>,
  setActiveGroup: React.Dispatch<React.SetStateAction<string | undefined>>
) => {
  const { setGridState, setActivePanelId } = useTabbedPaneStore();

  React.useEffect(() => {
    if (!api) return;

    const disposables = [
      api.onDidLayoutChange(() => {
        console.log("onDidLayoutChange");
        setGridState(api.toJSON());
      }),
      api.onDidAddPanel((event) => {
        console.log("onDidAddPanel");
        setPanels((prev) => [...prev, event.id]);
        addLogLine(`Panel Added ${event.id}`);
        setGridState(api.toJSON());
      }),
      api.onDidActivePanelChange((event) => {
        console.log("onDidActivePanelChange");
        setActivePanel(event?.id);
        setActivePanelId(event?.id);
        addLogLine(`Panel Activated ${event?.id}`);
      }),
      api.onDidRemovePanel((event) => {
        console.log("onDidRemovePanel");
        setPanels((prev) => prev.filter((id) => id !== event.id));
        addLogLine(`Panel Removed ${event?.id || "unknown"}`);
      }),
      api.onDidAddGroup((event) => {
        console.log("onDidAddGroup");
        setGroups((prev) => [...prev, event.id]);
        addLogLine(`Group Added ${event.id}`);
      }),
      api.onDidMovePanel((event) => {
        console.log("onDidMovePanel");
        addLogLine(`Panel Moved ${event.panel.id}`);
      }),
      api.onDidMaximizedGroupChange((event) => {
        console.log("onDidMaximizedGroupChange");
        addLogLine(`Group Maximized Changed ${event.group.api.id} [${event.isMaximized}]`);
      }),
      api.onDidRemoveGroup((event) => {
        console.log("onDidRemoveGroup");
        setGroups((prev) => prev.filter((id) => id !== event.id));
        addLogLine(`Group Removed ${event.id}`);
      }),
      api.onDidActiveGroupChange((event) => {
        console.log("onDidActiveGroupChange");
        setActiveGroup(event?.id);
        addLogLine(`Group Activated ${event?.id}`);
      }),
      api.onUnhandledDragOverEvent((event) => {
        console.log("onUnhandledDragOverEvent");
        event.accept();
      }),
    ];

    return () => {
      disposables.forEach((disposable) => disposable.dispose());
    };
  }, [api, addLogLine, setPanels, setGroups, setActivePanel, setActiveGroup, setGridState, setActivePanelId]);
};
