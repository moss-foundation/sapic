import { Scrollbar } from "@/lib/ui";
import { cn } from "@/utils";

import { PageViewProps } from "./types";

export const PageView = ({ children, className }: PageViewProps) => {
  return (
    <Scrollbar
      className={cn("background-(--moss-primary-background) relative flex grow flex-col", className)}
      classNames={{
        contentEl: "flex grow flex-col h-full",
      }}
    >
      {children}
    </Scrollbar>
  );
};
