import { CSSProperties, HTMLAttributes } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/tree-item";

interface DropIndicatorWithInstructionProps extends HTMLAttributes<HTMLDivElement> {
  instruction: Instruction;
  gap?: number;
  style?: CSSProperties;
}

export const DropIndicatorWithInstruction = ({
  instruction,
  gap = -1,
  style,
  ...props
}: DropIndicatorWithInstructionProps) => {
  const styleCss = {
    top: instruction.type === "reorder-above" ? gap : undefined,
    bottom: instruction.type === "reorder-below" ? gap : undefined,
  };

  return (
    <div
      className={cn("absolute z-100", {
        "h-[2px] w-full bg-blue-500": instruction.type === "reorder-above" || instruction.type === "reorder-below",
        "h-full w-full outline-2 -outline-offset-1 outline-blue-500": instruction.type === "make-child",
      })}
      style={styleCss}
      {...props}
    />
  );
};
