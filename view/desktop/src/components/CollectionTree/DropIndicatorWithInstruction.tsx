import { HTMLAttributes } from "react";

import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

interface DropIndicatorProps extends HTMLAttributes<HTMLDivElement> {
  instruction: Instruction | null;
  gap?: number;
  paddingLeft?: number;
  paddingRight?: number;
  isFolder?: boolean;
  depth?: number;
  isLastChild?: boolean;
  canDrop: boolean | null;
}

export const DropIndicatorWithInstruction = ({
  instruction,
  gap = 0,
  paddingLeft = 0,
  paddingRight = 0,
  isFolder = false,
  depth = 0,
  isLastChild = false,
  canDrop = true,
  ...props
}: DropIndicatorProps) => {
  if (!instruction) return null;

  const baseWidth = `calc(100% - ${paddingRight}px - ${paddingLeft}px)`;

  const reorderWidth = depth === 1 ? baseWidth : `calc(${baseWidth} - 16px)`;

  const leftOffset =
    depth === 1 ? 0 : !isFolder || instruction.operation === "reorder-before" ? 16 : isLastChild ? 0 : 16;
  const left = paddingLeft + leftOffset;

  let styles;

  if (instruction.operation === "combine" || !canDrop) {
    styles = {
      position: "absolute",
      height: "100%",
      width: "100%",
      top: 0,
      left: 0,
      zIndex: 7,
      backgroundColor: canDrop ? "var(--moss-success-background)" : "var(--moss-error-background)",
    };
  } else if (instruction.operation === "reorder-before" || instruction.operation === "reorder-after") {
    styles = {
      position: "absolute",
      height: "2px",
      backgroundColor: "var(--moss-primary)",
      [instruction.operation === "reorder-before" ? "top" : "bottom"]: gap,
      width: reorderWidth,
      left,
      zIndex: 7,
    };
  } else {
    return null;
  }

  return <div style={styles} {...props} />;
};
