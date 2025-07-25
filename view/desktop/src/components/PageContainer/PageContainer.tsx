import React from "react";
import { cn } from "@/utils";
import { PageContainerProps } from "./types";

export const PageContainer: React.FC<PageContainerProps> = ({ children, className }) => {
  return (
    <div className={cn("background-(--moss-primary-background) -m-3 flex h-full flex-col", className)}>{children}</div>
  );
};
