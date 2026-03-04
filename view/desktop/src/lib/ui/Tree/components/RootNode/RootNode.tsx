import { forwardRef, HTMLAttributes } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { ReorderDNDIndicator } from "../ReorderDNDIndicator";

interface RootNodeProps extends HTMLAttributes<HTMLUListElement> {
  children: React.ReactNode;
  className?: string;
  reorderInstruction?: Instruction | null;
  isDragging?: boolean;
}

export const RootNode = forwardRef<HTMLUListElement, RootNodeProps>(
  ({ children, className, reorderInstruction, isDragging, ...props }: RootNodeProps, ref) => {
    return (
      <ul
        ref={ref}
        className={cn(
          "group/TreeRootNode relative w-full list-none",
          {
            "opacity-50": isDragging,
          },
          className
        )}
        {...props}
      >
        <ReorderDNDIndicator reorderInstruction={reorderInstruction ?? null} />

        {children}
      </ul>
    );
  }
);
