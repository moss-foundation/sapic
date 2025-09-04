import { Scrollbar } from "@/lib/ui";
import { cn } from "@/utils";

import { PageContentProps } from "./types";

export const PageContent = ({ children, className }: PageContentProps) => {
  return (
    <Scrollbar>
      <div className={cn("h-full flex-1 p-3", className)}>{children}</div>
    </Scrollbar>
  );
};
