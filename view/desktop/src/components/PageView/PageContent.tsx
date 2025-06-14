import React from "react";
import { cn } from "@/utils";
import { Scrollbar } from "@/lib/ui";
import { PageContentProps } from "./types";

export const PageContent: React.FC<PageContentProps> = ({ children, className }) => {
  return (
    <Scrollbar className={cn("flex-1 overflow-auto", className)}>
      <div className="p-3">{children}</div>
    </Scrollbar>
  );
};
