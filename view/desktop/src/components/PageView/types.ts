import { IDockviewPanelProps } from "@/lib/moss-tabs/src";
import { ReactNode } from "react";

export interface PageViewProps {
  children: ReactNode;
  className?: string;
}

export interface PageHeaderProps {
  title: string;
  icon?: ReactNode;
  tabs?: ReactNode;
  toolbar?: ReactNode;
  className?: string;
  props?: IDockviewPanelProps;
}

export interface PageTabsProps {
  children: ReactNode;
  className?: string;
}

export interface PageToolbarProps {
  children: ReactNode;
  className?: string;
}

export interface PageContentProps {
  children: ReactNode;
  className?: string;
}
