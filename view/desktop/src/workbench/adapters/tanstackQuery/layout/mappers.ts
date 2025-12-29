import { SerializedDockview } from "moss-tabs";

import { LayoutStateInput, LayoutStateOutput } from "@/workbench/domains/layout/service";

export const mapUpdateLayoutInputToLayoutStateOutput = (
  oldLayout: LayoutStateOutput,
  input: LayoutStateInput
): LayoutStateOutput => {
  return {
    sidebarState: {
      ...oldLayout.sidebarState,
      ...(input.sidebarState ?? {}),
    },
    bottomPanelState: {
      ...oldLayout.bottomPanelState,
      ...(input.bottomPanelState ?? {}),
    },
    tabbedPaneState: input.tabbedPaneState?.gridState
      ? {
          gridState: input.tabbedPaneState.gridState as SerializedDockview,
        }
      : oldLayout.tabbedPaneState,
    activitybarState: {
      ...oldLayout.activitybarState,
      ...(input.activitybarState ?? {}),
    },
  };
};
