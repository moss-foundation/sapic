import { HTMLAttributes } from "react";

import { cn } from "@/utils";

interface RootDetailsProps extends HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode;
  className?: string;
}

export const RootDetails = ({ children, className, ...props }: RootDetailsProps) => {
  return (
    <div className={cn("group/TreeNodeDetails flex w-full min-w-0 items-center justify-between", className)} {...props}>
      {children}
    </div>
  );
};
