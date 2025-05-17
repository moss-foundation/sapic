import { HTMLAttributes } from "react";

import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/tree-item";

interface DropIndicatorProps extends HTMLAttributes<HTMLDivElement> {
  instruction: Instruction | null;
  gap?: number;
  paddingLeft?: number;
  paddingRight?: number;
  isFolder?: boolean;
  depth?: number;
  isLastChild?: boolean;
}

export const DropIndicatorWithInstruction = ({
  instruction,
  gap = -1,
  paddingLeft = 0,
  paddingRight = 0,
  isFolder = false,
  depth = 0,
  isLastChild = false,
  ...props
}: DropIndicatorProps) => {
  if (!instruction) return null;

  const baseWidth = `calc(100% - ${paddingRight}px - ${paddingLeft}px)`;

  const reorderWidth = depth === 1 ? baseWidth : `calc(${baseWidth} - 16px)`;

  const leftOffset = depth === 1 ? 0 : !isFolder || instruction.type === "reorder-above" ? 16 : isLastChild ? 0 : 16;
  const left = paddingLeft + leftOffset;

  let styles;

  switch (instruction.type) {
    case "make-child":
      styles = {
        position: "absolute",
        height: "100%",
        width: "100%",
        backgroundColor: "var(--moss-info-background)",
      };
      break;

    case "reorder-above":
      styles = {
        position: "absolute",
        height: "2px",
        backgroundColor: "var(--moss-primary)",
        top: depth === 1 ? 0 : gap,
        width: reorderWidth,
        left,
      };
      break;

    case "reorder-below":
      styles = {
        position: "absolute",
        height: "2px",
        backgroundColor: "var(--moss-primary)",
        bottom: gap,
        width: reorderWidth,
        left,
      };
      break;

    default:
      return null;
  }

  return <div style={styles} {...props} />;
};
