import { SerializedDockview } from "moss-tabs";

import { emptyGridState } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks/workspace";
import { sharedStorageService } from "@/lib/services/sharedStorage";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_TABBED_PANE_QUERY_KEY = "getTabbedPane";

interface TabbedPane {
  gridState: SerializedDockview;
}

const queryFn = async (activeWorkspaceId?: string): Promise<TabbedPane> => {
  if (!activeWorkspaceId) {
    return {
      gridState: emptyGridState,
    };
  }

  const gridState = (await sharedStorageService.getItem("gridState", activeWorkspaceId))
    ?.value as unknown as SerializedDockview;

  return {
    gridState: gridState ?? emptyGridState,
  };
};

export const useGetTabbedPane = () => {
  const { activeWorkspaceId } = useActiveWorkspace();

  return useQuery<TabbedPane, Error>({
    queryKey: [USE_GET_TABBED_PANE_QUERY_KEY, activeWorkspaceId],
    queryFn: () => queryFn(activeWorkspaceId),
  });
};
