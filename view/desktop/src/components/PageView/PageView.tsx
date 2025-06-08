import React from "react";
import { cn } from "@/utils";
import { PageViewProps } from "./types";

export const PageView: React.FC<PageViewProps> = ({ children, className }) => {
  return <div className={cn("flex h-full flex-col bg-gray-50 dark:bg-stone-900", className)}>{children}</div>;
};
