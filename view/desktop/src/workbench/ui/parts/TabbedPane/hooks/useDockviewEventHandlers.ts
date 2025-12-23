import { SerializedDockview } from "moss-tabs";
import React from "react";

import { useActiveWorkspace } from "@/hooks/workspace/derived/useActiveWorkspace";
import { useUpdateLayout } from "@/workbench/adapters";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

interface UseTabbedPaneEventHandlersProps {
  canPragmaticDrop: boolean;
}

export const useTabbedPaneEventHandlers = ({ canPragmaticDrop }: UseTabbedPaneEventHandlersProps) => {
  const { setActivePanelId, api } = useTabbedPaneStore();
  const { activeWorkspaceId, hasActiveWorkspace } = useActiveWorkspace();

  const { mutate: updateLayout } = useUpdateLayout();

  React.useEffect(() => {
    if (!api || !activeWorkspaceId) return;

    const disposables = [
      api.onDidLayoutChange(() => {
        if (!hasActiveWorkspace) return;

        const newGridState = api.toJSON();
        updateLayout({
          layout: {
            tabbedPaneState: {
              gridState: newGridState as unknown as SerializedDockview,
            },
          },
          workspaceId: activeWorkspaceId,
        });
      }),
      api.onDidActivePanelChange((event) => {
        setActivePanelId(event?.id);
      }),
      api.onUnhandledDragOverEvent((event) => {
        event.accept();
      }),
      api.onWillShowOverlay((event) => {
        if (canPragmaticDrop) return;
        event.preventDefault();
      }),
    ];

    return () => {
      disposables.forEach((disposable) => disposable.dispose());
    };
  }, [api, setActivePanelId, canPragmaticDrop, updateLayout, activeWorkspaceId, hasActiveWorkspace]);
};
