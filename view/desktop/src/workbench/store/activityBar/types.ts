import { ComponentPropsWithoutRef, ReactNode } from "react";

export interface ActivityBarItemProps extends ComponentPropsWithoutRef<"button"> {
  id: string;
  icon: ReactNode;
  iconActive?: ReactNode;
  title: string;
  order: number;
  isVisible?: boolean;
  isDraggable?: boolean;
}

export interface ActivityBarStore {
  items: ActivityBarItemProps[];
  setItems: (items: ActivityBarItemProps[]) => void;
}
