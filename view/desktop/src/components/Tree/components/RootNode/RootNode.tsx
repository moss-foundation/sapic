import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DropIndicatorForDir } from "../DropIndicatorForDir";
import { DropIndicatorForTrigger } from "../DropIndicatorForTrigger";

interface RootNodeProps {
  children: React.ReactNode;
  className?: string;
  isChildDropBlocked: boolean | null;
  instruction: Instruction | null;
  dropTargetRootRef?: React.RefObject<HTMLLIElement>;
}

export const RootNode = ({
  children,
  className,
  isChildDropBlocked,
  instruction,
  dropTargetRootRef,
  ...props
}: RootNodeProps) => {
  return (
    <li ref={dropTargetRootRef} className={cn("group/TreeRootNode relative w-full list-none", className)} {...props}>
      <DropIndicatorForDir isChildDropBlocked={isChildDropBlocked} instruction={instruction} />
      <DropIndicatorForTrigger instruction={instruction} />

      {children}
    </li>
  );
};
