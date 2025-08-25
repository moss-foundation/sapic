import { Scrollbar } from "@/lib/ui";
import { cn } from "@/utils";

import { PageContentProps } from "./types";

export const PageContent = ({ children, className }: PageContentProps) => {
  return (
    <Scrollbar
      classNames={{
        contentWrapper: "h-full",
        contentEl: "h-full",
      }}
    >
      <div className={cn("h-full flex-1 p-3", className)}>{children}</div>
    </Scrollbar>
  );
};
