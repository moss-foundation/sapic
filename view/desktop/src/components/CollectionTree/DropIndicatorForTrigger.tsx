import { HTMLAttributes } from "react";

import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

interface DropIndicatorProps extends HTMLAttributes<HTMLDivElement> {
  instruction: Instruction | null;
  gap?: number;
  paddingLeft?: number;
  paddingRight?: number;
  depth?: number;
  isLastChild?: boolean;
  height?: number;
}

export const DropIndicatorForTrigger = ({
  instruction,
  gap = 0,
  paddingLeft = 0,
  paddingRight = 0,
  depth = 0,
  isLastChild = false,
  height = 1,
  ...props
}: DropIndicatorProps) => {
  if (!instruction || instruction.blocked || instruction.operation === "combine") return null;

  const baseWidth = `calc(100% - ${paddingRight}px - ${paddingLeft}px)`;
  const reorderWidth = depth === 1 ? baseWidth : `calc(${baseWidth} - 16px)`;

  const leftOffset =
    paddingLeft + (depth === 1 ? 0 : instruction.operation === "reorder-before" ? 16 : isLastChild ? 0 : 16);

  const gapOffset = -(gap + height / 2);

  return (
    <div
      style={{
        position: "absolute",
        height: "1px",
        backgroundColor: "var(--moss-primary)",
        [instruction.operation === "reorder-before" ? "top" : "bottom"]: gapOffset,
        width: reorderWidth,
        left: leftOffset,
        zIndex: 5,
      }}
      {...props}
    />
  );
};
