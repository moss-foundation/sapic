import { ComponentPropsWithoutRef, ReactNode } from "react";

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
