import { ReactNode } from "react";

export interface TabItem {
  id: string;
  label: ReactNode;
  content: ReactNode;
  icon?: ReactNode;
}
