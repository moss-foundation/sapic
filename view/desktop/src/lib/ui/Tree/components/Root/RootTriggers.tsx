import { HTMLAttributes, ReactNode } from "react";

import { cn } from "@/utils/cn";

interface RootTriggersProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
  className?: string;
}

export const RootTriggers = ({ children, className, ...props }: RootTriggersProps) => {
  return (
    <div className={cn("flex grow items-center gap-1", className)} {...props}>
      {children}
    </div>
  );
};
