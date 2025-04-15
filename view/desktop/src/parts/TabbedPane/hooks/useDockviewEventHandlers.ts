import React from "react";

import { useDockviewStore } from "@/store/Dockview";
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
  const dockviewStore = useDockviewStore();

  const { setGridState } = useTabbedPaneStore();
  React.useEffect(() => {
    if (!api) return;

    const disposables = [
      api.onDidLayoutChange(() => {
        setGridState(api.toJSON());
      }),
      api.onDidAddPanel((event) => {
        setPanels((prev) => [...prev, event.id]);
        addLogLine(`Panel Added ${event.id}`);
        setGridState(api.toJSON());
      }),
      api.onDidActivePanelChange((event) => {
        setActivePanel(event?.id);
        dockviewStore.setCurrentActivePanelId(event?.id || undefined);
        dockviewStore.setCurrentActiveTreeId(event?.params?.treeId || undefined);
        addLogLine(`Panel Activated ${event?.id}`);
      }),
      api.onDidRemovePanel((event) => {
        setPanels((prev) => prev.filter((id) => id !== event.id));
        addLogLine(`Panel Removed ${event?.id || "unknown"}`);
      }),
      api.onDidAddGroup((event) => {
        setGroups((prev) => [...prev, event.id]);
        addLogLine(`Group Added ${event.id}`);
      }),
      api.onDidMovePanel((event) => {
        addLogLine(`Panel Moved ${event.panel.id}`);
      }),
      api.onDidMaximizedGroupChange((event) => {
        addLogLine(`Group Maximized Changed ${event.group.api.id} [${event.isMaximized}]`);
      }),
      api.onDidRemoveGroup((event) => {
        setGroups((prev) => prev.filter((id) => id !== event.id));
        addLogLine(`Group Removed ${event.id}`);
      }),
      api.onDidActiveGroupChange((event) => {
        setActiveGroup(event?.id);
        addLogLine(`Group Activated ${event?.id}`);
      }),
      api.onUnhandledDragOverEvent((event) => {
        event.accept();
      }),
    ];

    return () => {
      disposables.forEach((disposable) => disposable.dispose());
    };
  }, [api, addLogLine, setPanels, setGroups, setActivePanel, setActiveGroup, dockviewStore, setGridState]);
};
