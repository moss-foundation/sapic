import { useMemo } from "react";

import { useBatchGetActivityBarItemState } from "@/workbench/adapters/tanstackQuery/activityBarItemState/useBatchGetActivityBarItemState";

import { placeholderActivityBarFirstItems } from "../components/placeholder";
import { ActivityBarItem } from "../types";

export const useSyncedActivityBarFirstItems = () => {
  const { data: activityBarItemStates, isLoading: isLoadingActivityBarItemStates } = useBatchGetActivityBarItemState(
    placeholderActivityBarFirstItems.map((item) => item.id)
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
