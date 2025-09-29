import { ReactNode } from "react";

export interface TabItemProps {
  id: string;
  label: ReactNode;
  content: ReactNode;
  icon?: ReactNode;
}

export type ProviderIcon = "github" | "gitlab" | "postman" | "insomnia";
