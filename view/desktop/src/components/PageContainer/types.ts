import { ReactNode } from "react";

export interface PageContainerProps {
  children: ReactNode;
  className?: string;
}

export interface PageContainerHeaderProps {
  children?: ReactNode;
  className?: string;
}

export interface PageContainerContentProps {
  children: ReactNode;
  className?: string;
}

export interface TabItem {
  id: string;
  label: ReactNode;
  content: ReactNode;
  icon?: ReactNode;
}

export interface PageContainerWithTabsProps {
  tabs: TabItem[];
  activeTabId: string;
  onTabChange: (tabId: string) => void;
  className?: string;
}
