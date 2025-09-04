import { HTMLAttributes } from "react";

import { cn } from "@/utils";

interface RootNodeControlsProps extends HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode;
  className?: string;
}

export const RootNodeControls = ({ children, className, ...props }: RootNodeControlsProps) => {
  return (
    <div
      className={cn("group/TreeRootNodeControls flex w-full min-w-0 items-center justify-between", className)}
      {...props}
    >
      {children}
    </div>
  );
};
