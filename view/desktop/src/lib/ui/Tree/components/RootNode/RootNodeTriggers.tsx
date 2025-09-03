import { HTMLAttributes, ReactNode } from "react";

import { cn } from "@/utils/cn";

interface RootNodeTriggersProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
  className?: string;
}

export const RootNodeTriggers = ({ children, className, ...props }: RootNodeTriggersProps) => {
  return (
    <div className={cn("flex grow items-center gap-1", className)} {...props}>
      {children}
    </div>
  );
};
