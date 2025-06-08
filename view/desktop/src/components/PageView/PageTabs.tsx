import React from "react";
import { cn } from "@/utils";
import { PageTabsProps } from "./types";

export const PageTabs: React.FC<PageTabsProps> = ({ children, className }) => {
  return <div className={cn("flex items-center gap-0.5", className)}>{children}</div>;
};
