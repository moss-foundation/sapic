import { forwardRef, HTMLAttributes, ReactNode } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { CombineDNDIndicator } from "../CombineDNDIndicator";

interface NodeProps extends HTMLAttributes<HTMLLIElement> {
  children: ReactNode;
  combineInstruction?: Instruction | null;
  className?: string;
  isDragging?: boolean;
}

export const Node = forwardRef<HTMLLIElement, NodeProps>(
  ({ children, className, combineInstruction, isDragging, ...props }: NodeProps, ref) => {
    return (
      <li ref={ref} className={cn("relative", className, { "opacity-50": isDragging })} {...props}>
        <CombineDNDIndicator combineInstruction={combineInstruction} />

        {children}
      </li>
    );
  }
);
