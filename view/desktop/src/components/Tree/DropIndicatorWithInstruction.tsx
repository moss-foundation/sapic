import { HTMLAttributes } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/tree-item";

interface DropIndicatorWithInstructionProps extends HTMLAttributes<HTMLDivElement> {
  instruction: Instruction;
}

export const DropIndicatorWithInstruction = ({ instruction, ...props }: DropIndicatorWithInstructionProps) => {
  return (
    <div
      className={cn("absolute z-100", {
        "top-0 h-px w-full bg-blue-500": instruction.type === "reorder-above",
        "bottom-0 h-px w-full bg-blue-500": instruction.type === "reorder-below",
        "h-full w-full outline -outline-offset-1 outline-blue-500": instruction.type === "make-child",
      })}
      {...props}
    />
  );
};
