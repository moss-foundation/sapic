import { cn } from "@/utils";

import { PageContainerProps } from "./types";

export const PageContainer = ({ children, className }: PageContainerProps) => {
  return (
    <div className={cn("flex h-full flex-col overflow-auto rounded-md border border-(--moss-border-color)", className)}>
      {children}
    </div>
  );
};
