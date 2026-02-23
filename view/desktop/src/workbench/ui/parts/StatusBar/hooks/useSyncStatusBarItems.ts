import { useMemo } from "react";

import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { useBatchGetStatusBarItemState } from "@/workbench/adapters/tanstackQuery";

import { placeholderStatusBarButtons } from "../components/placeholderStatusBarButtons";
import { StatusBarItem } from "../types";

export const useSyncStatusBarItems = () => {
  const ids = placeholderStatusBarButtons.map((item) => item.id);
  const { data: orders, isLoading } = useBatchGetStatusBarItemState(ids);

  const items = useMemo((): StatusBarItem[] => {
    const list = placeholderStatusBarButtons.map((item, i) => ({
      ...item,
      order: orders?.[i] ?? -1,
    })) as unknown as StatusBarItem[];
    return sortObjectsByOrder(list, "label");
  }, [orders]);

  return { items, isLoading };
};
