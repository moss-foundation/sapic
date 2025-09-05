import { forwardRef, HTMLAttributes, ReactNode } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DropIndicatorForDir } from "../DropIndicatorForDir";

interface NodeProps extends HTMLAttributes<HTMLLIElement> {
  children: ReactNode;
  isChildDropBlocked?: boolean | null;
  dropIndicatorInstruction?: Instruction | null;
  className?: string;
}

export const Node = forwardRef<HTMLLIElement, NodeProps>(
  ({ children, className, isChildDropBlocked, dropIndicatorInstruction, ...props }: NodeProps, ref) => {
    return (
      <li ref={ref} className={cn("relative", className)} {...props}>
        <DropIndicatorForDir isChildDropBlocked={isChildDropBlocked} instruction={dropIndicatorInstruction ?? null} />

        {children}
      </li>
    );
  }
);
