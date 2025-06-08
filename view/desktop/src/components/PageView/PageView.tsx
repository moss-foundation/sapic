import React from "react";
import { cn } from "@/utils";
import { PageViewProps } from "./types";

export const PageView: React.FC<PageViewProps> = ({ children, className }) => {
  return <div className={cn("flex h-full flex-col bg-gray-50", className)}>{children}</div>;
};
