import { HTMLAttributes } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/tree-item";

interface DropIndicatorWithInstructionProps extends HTMLAttributes<HTMLDivElement> {
  instruction: Instruction | null;
  gap?: number;
  paddingLeft?: number;
  paddingRight?: number;
  isFolder?: boolean;
}

export const DropIndicatorWithInstruction = ({
  instruction,
  gap = -1,
  paddingLeft = 0,
  paddingRight = 0,
  isFolder = false,
  ...props
}: DropIndicatorWithInstructionProps) => {
  if (!instruction) {
    return null;
  }

  const styleCss = {
    position: "absolute" as const,
    height: instruction.type === "reorder-above" || instruction.type === "reorder-below" ? "2px" : "100%",
    backgroundColor:
      instruction.type === "reorder-above" || instruction.type === "reorder-below"
        ? "var(--moss-primary)"
        : "transparent",
    top: instruction.type === "reorder-above" ? gap : undefined,
    bottom: instruction.type === "reorder-below" ? gap : undefined,
    width: `calc(100% - ${paddingRight}px - ${paddingLeft}px - ${isFolder ? 0 : 16}px)`,
    left: paddingLeft + (isFolder ? 0 : 16),
  };

  return (
    <div
      className={cn({
        "h-full w-full outline-2 -outline-offset-2 outline-(--moss-primary)": instruction.type === "make-child",
      })}
      style={styleCss}
      {...props}
    />
  );
};
