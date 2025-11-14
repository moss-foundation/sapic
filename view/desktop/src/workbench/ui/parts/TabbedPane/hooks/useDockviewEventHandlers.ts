import { SerializedDockview } from "moss-tabs";
import React from "react";

import { useUpdateLayout } from "@/hooks/workbench/layout/useUpdateLayout";
import { useActiveWorkspace } from "@/hooks/workspace/derived/useActiveWorkspace";
import { useTabbedPaneStore } from "@/store/tabbedPane";

interface UseTabbedPaneEventHandlersProps {
  canPragmaticDrop: boolean;
}

export const useTabbedPaneEventHandlers = ({ canPragmaticDrop }: UseTabbedPaneEventHandlersProps) => {
  const { setActivePanelId, api } = useTabbedPaneStore();
  const { activeWorkspaceId, hasActiveWorkspace } = useActiveWorkspace();

  const { mutate: updateLayout } = useUpdateLayout();

  React.useEffect(() => {
    if (!api) return;

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
