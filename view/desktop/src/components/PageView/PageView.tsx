import { cn } from "@/utils";

import { PageViewProps } from "./types";

export const PageView = ({ children, className }: PageViewProps) => {
  return (
    <div className={cn("background-(--moss-primary-background) relative flex h-full flex-col", className)}>
      {children}
    </div>
  );
};
