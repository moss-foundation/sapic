import { HTMLAttributes, ReactNode, RefObject } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { CombineDNDIndicator } from "../CombineDNDIndicator";

interface ListProps extends HTMLAttributes<HTMLDivElement> {
  combineInstruction?: Instruction | null;

  ref?: RefObject<HTMLDivElement | null>;
  children: ReactNode;
  className?: string;
}

export const List = ({ children, className, ref, combineInstruction, ...props }: ListProps) => {
  return (
    <section className={cn("relative", className)} ref={ref} {...props}>
      {combineInstruction && <CombineDNDIndicator combineInstruction={combineInstruction} />}

      {children}
    </section>
  );
};
