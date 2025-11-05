import { DockviewApi, SerializedDockview } from "moss-tabs";
import React from "react";

import { useTabbedPaneStore } from "@/store/tabbedPane";

interface UseTabbedPaneEventHandlersProps {
  api: DockviewApi | undefined;
  setGridState: (state: SerializedDockview) => void;
  canDrop: boolean;
}

export const useTabbedPaneEventHandlers = ({ api, setGridState, canDrop }: UseTabbedPaneEventHandlersProps) => {
  const { setActivePanelId } = useTabbedPaneStore();

  React.useEffect(() => {
    if (!api) return;

    const disposables = [
      api.onDidLayoutChange(() => {
        setGridState(api?.toJSON() as SerializedDockview);
      }),
      api.onDidActivePanelChange((event) => {
        setActivePanelId(event?.id);
      }),
      api.onUnhandledDragOverEvent((event) => {
        event.accept();
      }),
      api.onWillShowOverlay((event) => {
        if (canDrop) return;
        event.preventDefault();
      }),
    ];

    return () => {
      disposables.forEach((disposable) => disposable.dispose());
    };
  }, [api, setActivePanelId, canDrop, setGridState]);
};
