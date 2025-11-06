import { SerializedDockview } from "moss-tabs";
import React from "react";

import { useUpdateTabbedPane } from "@/hooks/sharedStorage/layout/tabbedPane/useUpdateTabbedPane";
import { useTabbedPaneStore } from "@/store/tabbedPane";

interface UseTabbedPaneEventHandlersProps {
  canDrop: boolean;
}

export const useTabbedPaneEventHandlers = ({ canDrop }: UseTabbedPaneEventHandlersProps) => {
  const { setActivePanelId, api } = useTabbedPaneStore();

  const { mutate: updateTabbedPane } = useUpdateTabbedPane();

  React.useEffect(() => {
    if (!api) return;

    const disposables = [
      api.onDidLayoutChange(() => {
        const newGridState = api.toJSON();
        updateTabbedPane(newGridState as unknown as SerializedDockview);
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
  }, [api, setActivePanelId, canDrop, updateTabbedPane]);
};
