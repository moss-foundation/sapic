import { HTMLAttributes } from "react";

import { cn } from "@/utils";

import { DirDepthIndicator } from "../DirDepthIndicator";

interface NodeChildrenProps extends HTMLAttributes<HTMLUListElement> {
  children?: React.ReactNode;
  className?: string;
  dirDepthIndicatorOffset: number;
}

export const NodeChildren = ({ children, className, dirDepthIndicatorOffset, ...props }: NodeChildrenProps) => {
  return (
    <ul className={cn("relative h-full", className)} {...props}>
      <DirDepthIndicator offset={dirDepthIndicatorOffset} />

      {children}
    </ul>
  );
};
