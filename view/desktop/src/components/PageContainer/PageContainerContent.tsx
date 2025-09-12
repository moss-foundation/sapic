import React from "react";

import { Scrollbar } from "@/lib/ui";
import { cn } from "@/utils";

import { PageContainerContentProps } from "./types";

export const PageContainerContent: React.FC<PageContainerContentProps> = ({ children, className }) => {
  return (
    <Scrollbar className={cn("flex-1 overflow-auto", className)}>
      <div className="p-3">{children}</div>
    </Scrollbar>
  );
};
