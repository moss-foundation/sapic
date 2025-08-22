import { Scrollbar } from "@/lib/ui";
import { cn } from "@/utils";

import { PageContentProps } from "./types";

export const PageContent = ({ children, className }: PageContentProps) => {
  return <Scrollbar className={cn("flex-1 p-3", className)}>{children}</Scrollbar>;
};
