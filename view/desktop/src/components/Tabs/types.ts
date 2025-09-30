import { ReactNode } from "react";

import { Icons } from "@/lib/ui";

export interface TabItemProps {
  id: string;
  label: ReactNode;
  content: ReactNode;
  icon?: Icons;
  count?: number;
}

export type ProviderIcon = "github" | "gitlab" | "postman" | "insomnia";
