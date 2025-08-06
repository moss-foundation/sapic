import { HTMLAttributes } from "react";

import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

interface DropIndicatorProps extends HTMLAttributes<HTMLDivElement> {
  instruction: Instruction | null;
  gap?: number;
  paddingLeft?: number;
  paddingRight?: number;
  depth?: number;
  isLastChild?: boolean;
}

export const DropIndicatorForTrigger = ({
  instruction,
  gap = 0,
  paddingLeft = 0,
  paddingRight = 0,
  depth = 0,
  isLastChild = false,
  ...props
}: DropIndicatorProps) => {
  console.log(instruction);

  if (!instruction || instruction.blocked || instruction.operation === "combine") return null;

  const baseWidth = `calc(100% - ${paddingRight}px - ${paddingLeft}px)`;

  const reorderWidth = depth === 1 ? baseWidth : `calc(${baseWidth} - 16px)`;

  const leftOffset = depth === 1 ? 0 : instruction.operation === "reorder-before" ? 16 : isLastChild ? 0 : 16;
  const left = paddingLeft + leftOffset;

  return (
    <div
      style={{
        position: "absolute",
        height: "2px",
        backgroundColor: "var(--moss-primary)",
        [instruction.operation === "reorder-before" ? "top" : "bottom"]: gap,
        width: reorderWidth,
        left,
      }}
      {...props}
    />
  );
};
