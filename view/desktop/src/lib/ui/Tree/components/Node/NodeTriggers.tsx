import { HTMLAttributes, ReactNode } from "react";

import { cn } from "@/utils";

interface NodeTriggersProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
}

export const NodeTriggers = ({ children, className, ...props }: NodeTriggersProps) => {
  return (
    <div className={cn("relative z-10 flex h-full grow items-center gap-1.5", className)} {...props}>
      {children}
    </div>
  );
};
