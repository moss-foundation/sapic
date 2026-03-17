import { HTMLAttributes } from "react";

import { cn } from "@/utils";

import { DirDepthIndicator } from "../DirDepthIndicator";

interface NodeChildrenProps extends HTMLAttributes<HTMLUListElement> {
  children?: React.ReactNode;
  className?: string;
  depth: number;
  nodeOffset: number;
}

export const NodeChildren = ({ children, className, depth, nodeOffset, ...props }: NodeChildrenProps) => {
  const offset = depth * nodeOffset;

  return (
    <ul className={cn("relative h-full", className)} {...props}>
      <DirDepthIndicator offset={offset} />

      {children}
    </ul>
  );
};
