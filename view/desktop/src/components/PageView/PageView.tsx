import React from "react";

import { cn } from "@/utils";

import { PageViewProps } from "./types";

export const PageView: React.FC<PageViewProps> = ({ children, className }) => {
  return (
    <div className={cn("background-(--moss-primary-background) relative flex h-full flex-col", className)}>
      {children}
    </div>
  );
};
