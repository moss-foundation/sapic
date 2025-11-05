import React from "react";

import { useActiveWorkspace } from "@/hooks";
import { sharedStorageService } from "@/lib/services/sharedStorageService";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { JsonValue } from "@repo/moss-bindingutils";

interface UseTabbedPaneEventHandlersProps {
  canDrop: boolean;
}

export const useTabbedPaneEventHandlers = ({ canDrop }: UseTabbedPaneEventHandlersProps) => {
  const { setActivePanelId, api, setGridState } = useTabbedPaneStore();

  const { activeWorkspaceId } = useActiveWorkspace();

  React.useEffect(() => {
    if (!api) return;

    const disposables = [
      api.onDidLayoutChange(() => {
        if (!activeWorkspaceId) return;

        const newGridState = api.toJSON();

        sharedStorageService.putItem("gridState", newGridState as unknown as JsonValue, activeWorkspaceId);
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
  }, [api, setActivePanelId, canDrop, setGridState, activeWorkspaceId]);
};
