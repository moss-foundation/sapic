import React from "react";
import { cn } from "@/utils";
import { PageToolbarProps } from "./types";

export const PageToolbar: React.FC<PageToolbarProps> = ({ children, className }) => {
  return <div className={cn("flex items-center gap-1", className)}>{children}</div>;
};
