import { HTMLAttributes, ReactNode, RefObject } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { ListInstruction } from "./ListInstruction";

interface ListProps extends HTMLAttributes<HTMLDivElement> {
  instruction?: Instruction | null;

  ref?: RefObject<HTMLDivElement | null>;
  children: ReactNode;
  className?: string;
}

export const List = ({ children, className, ref, instruction, ...props }: ListProps) => {
  return (
    <section className={cn("relative h-full", className)} ref={ref} {...props}>
      {instruction && <ListInstruction instruction={instruction} />}

      {children}
    </section>
  );
};
