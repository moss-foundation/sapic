import { HTMLAttributes } from "react";

import { cn } from "@/utils";

interface RootNodeDetailsProps extends HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode;
  className?: string;
}

export const RootNodeDetails = ({ children, className, ...props }: RootNodeDetailsProps) => {
  return (
    <div
      className={cn("group/TreeRootNodeDetails flex w-full min-w-0 items-center justify-between", className)}
      {...props}
    >
      {children}
    </div>
  );
};
