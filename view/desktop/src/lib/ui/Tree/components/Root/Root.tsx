import { forwardRef, HTMLAttributes } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { ReorderDNDIndicator } from "../ReorderDNDIndicator";

interface RootProps extends HTMLAttributes<HTMLUListElement> {
  children: React.ReactNode;
  className?: string;
  reorderInstruction?: Instruction | null;
  isDragging?: boolean;
}

export const Root = forwardRef<HTMLUListElement, RootProps>(
  ({ children, className, reorderInstruction, isDragging, ...props }: RootProps, ref) => {
    return (
      <ul
        ref={ref}
        className={cn(
          "group/TreeRoot relative w-full list-none",
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
