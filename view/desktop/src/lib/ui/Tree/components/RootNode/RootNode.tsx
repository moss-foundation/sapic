import { forwardRef, HTMLAttributes } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DropIndicatorForDir } from "../DropIndicatorForDir";
import { DropIndicatorForTrigger } from "../DropIndicatorForTrigger";

interface RootNodeProps extends HTMLAttributes<HTMLUListElement> {
  children: React.ReactNode;
  className?: string;
  isChildDropBlocked: boolean | null;
  instruction: Instruction | null;
}

export const RootNode = forwardRef<HTMLUListElement, RootNodeProps>(
  ({ children, className, isChildDropBlocked, instruction, ...props }: RootNodeProps, ref) => {
    return (
      <ul ref={ref} className={cn("group/TreeRootNode relative w-full list-none", className)} {...props}>
        <DropIndicatorForDir isChildDropBlocked={isChildDropBlocked} instruction={instruction} />
        <DropIndicatorForTrigger instruction={instruction} />

        {children}
      </ul>
    );
  }
);
