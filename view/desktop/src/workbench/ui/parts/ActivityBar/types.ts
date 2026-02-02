import { ComponentPropsWithoutRef, ReactNode } from "react";
import { ACTIVITY_BAR_BUTTON_DND_TYPE } from "./constants";

export interface ActivityBarItemData {
  id: string;
  title: string;
  icon: ReactNode;
  iconActive?: ReactNode;
}

export interface ActivityBarItemState {
  id: string;
  order: number;
}

export interface ActivityBarItem
  extends ActivityBarItemData, ActivityBarItemState, Omit<ComponentPropsWithoutRef<"button">, "id" | "title"> {}

export interface ActivityBarButtonProps
  extends ActivityBarItem, ActivityBarItemState, Omit<ComponentPropsWithoutRef<"button">, "id" | "title"> {
  isDraggable: boolean;
}

export interface ActivityBarButtonDragData {
  type: typeof ACTIVITY_BAR_BUTTON_DND_TYPE;
  data: {
    id: string;
    order: number;
  };
  [key: string]: unknown;
}
