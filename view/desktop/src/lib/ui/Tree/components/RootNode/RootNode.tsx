import { forwardRef, HTMLAttributes } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DropIndicatorForCombine } from "../DropIndicatorForCombine";
import { DropIndicatorForDir } from "../DropIndicatorForDir";
import { DropIndicatorForTrigger } from "../DropIndicatorForTrigger";

interface RootNodeProps extends HTMLAttributes<HTMLUListElement> {
  children: React.ReactNode;
  className?: string;
  isChildDropBlocked?: boolean | null;
  instruction?: Instruction | null;
  dropIndicatorFullWidth?: boolean;
  combineInstruction?: Instruction | null;
  isDragging?: boolean;
}

export const RootNode = forwardRef<HTMLUListElement, RootNodeProps>(
  (
    {
      children,
      className,
      isChildDropBlocked,
      instruction,
      dropIndicatorFullWidth = false,
      combineInstruction,
      isDragging,
      ...props
    }: RootNodeProps,
    ref
  ) => {
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
        <DropIndicatorForDir isChildDropBlocked={isChildDropBlocked} instruction={instruction ?? null} />
        <DropIndicatorForTrigger instruction={instruction ?? null} fullWidth={dropIndicatorFullWidth} />
        {combineInstruction && <DropIndicatorForCombine instruction={combineInstruction} />}

        {children}
      </ul>
    );
  }
);
