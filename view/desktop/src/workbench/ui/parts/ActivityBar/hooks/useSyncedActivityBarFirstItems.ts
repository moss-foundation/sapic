import { useCurrentWorkspace } from "@/hooks/workspace/derived/useCurrentWorkspace";
import { useBatchGetActivityBarItemState } from "@/workbench/adapters/tanstackQuery/activityBarItemState/useBatchGetActivityBarItemState";
import { useMemo } from "react";
import { placeholderActivityBarFirstItems } from "../components/placeholder";
import { ActivityBarItem } from "../types";

export const useSyncedActivityBarFirstItems = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: activityBarItemStates, isLoading: isLoadingActivityBarItemStates } = useBatchGetActivityBarItemState(
    placeholderActivityBarFirstItems.map((item) => item.id),
    currentWorkspaceId
  );

  const items: ActivityBarItem[] = useMemo(() => {
    return placeholderActivityBarFirstItems
      .map((item) => {
        const activityBarItemState = activityBarItemStates?.find(
          (activityBarItemState) => activityBarItemState.id === item.id
        );
        return {
          ...item,
          order: activityBarItemState?.order ?? -1,
        };
      })
      .sort((a, b) => a.order - b.order);
  }, [activityBarItemStates]);

  return { items, isLoadingActivityBarItemStates };
};
