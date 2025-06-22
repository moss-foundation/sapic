import React from "react";
import { cn } from "@/utils";
import { PageContainerToolbarProps } from "./types";

export const PageContainerToolbar: React.FC<PageContainerToolbarProps> = ({ children, className }) => {
  return <div className={cn("flex items-center gap-1", className)}>{children}</div>;
};
