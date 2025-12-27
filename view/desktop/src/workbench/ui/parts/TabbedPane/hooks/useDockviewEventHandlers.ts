import { SerializedDockview } from "moss-tabs";
import React from "react";

import { useCurrentWorkspace } from "@/hooks/workspace/derived/useCurrentWorkspace";
import { useUpdateLayout } from "@/workbench/adapters";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

interface UseTabbedPaneEventHandlersProps {
  canPragmaticDrop: boolean;
}

export const useTabbedPaneEventHandlers = ({ canPragmaticDrop }: UseTabbedPaneEventHandlersProps) => {
  const { setActivePanelId, api } = useTabbedPaneStore();
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { mutate: updateLayout } = useUpdateLayout();

  React.useEffect(() => {
    if (!api || !currentWorkspaceId) return;

    const disposables = [
      api.onDidLayoutChange(() => {
        const newGridState = api.toJSON();
        updateLayout({
          layout: {
            tabbedPaneState: {
              gridState: newGridState as unknown as SerializedDockview,
            },
          },
          workspaceId: currentWorkspaceId,
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
  }, [api, setActivePanelId, canPragmaticDrop, updateLayout, currentWorkspaceId]);
};
