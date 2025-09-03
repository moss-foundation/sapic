import { HTMLAttributes } from "react";

import { cn } from "@/utils";

import { DirDepthIndicator } from "../DirDepthIndicator";

interface NodeChildrenProps extends HTMLAttributes<HTMLUListElement> {
  children?: React.ReactNode;
  className?: string;
  depth: number;
}

export const NodeChildren = ({ children, className, depth, ...props }: NodeChildrenProps) => {
  return (
    <ul className={cn("relative h-full", className)} {...props}>
      <DirDepthIndicator depth={depth} />

      {children}
    </ul>
  );
};
