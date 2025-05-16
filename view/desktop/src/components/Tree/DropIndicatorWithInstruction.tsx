import { HTMLAttributes } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/tree-item";

interface DropIndicatorWithInstructionProps extends HTMLAttributes<HTMLDivElement> {
  instruction: Instruction | null;
  gap?: number;
  paddingLeft?: number;
  paddingRight?: number;
  isFolder?: boolean;
  depth?: number;
}

export const DropIndicatorWithInstruction = ({
  instruction,
  gap = -1,
  paddingLeft = 0,
  paddingRight = 0,
  isFolder = false,
  depth = 0,
  ...props
}: DropIndicatorWithInstructionProps) => {
  if (!instruction) {
    return null;
  }

  console.log(depth);

  const styleCss = {
    position: "absolute" as const,
    height: instruction.type === "reorder-above" || instruction.type === "reorder-below" ? "2px" : "100%",
    backgroundColor:
      instruction.type === "reorder-above" || instruction.type === "reorder-below"
        ? "var(--moss-primary)"
        : "transparent",
    top: instruction.type === "reorder-above" ? (depth === 1 ? 0 : gap) : undefined,
    bottom: instruction.type === "reorder-below" ? gap : undefined,
    width:
      depth === 1
        ? ` calc(100% - ${paddingRight}px - ${paddingLeft}px)`
        : `calc(100% - ${paddingRight}px - ${paddingLeft}px - ${isFolder ? 0 : 16 * (instruction.type === "reorder-above" ? 2 : 1)}px)`,
    left: depth === 1 ? paddingLeft : paddingLeft + (isFolder ? (instruction.type === "reorder-above" ? 16 : 0) : 16),
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
