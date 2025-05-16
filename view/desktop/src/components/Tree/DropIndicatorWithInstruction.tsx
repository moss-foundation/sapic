import { HTMLAttributes } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/tree-item";

interface DropIndicatorProps extends HTMLAttributes<HTMLDivElement> {
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
}: DropIndicatorProps) => {
  if (!instruction) return null;

  const isReorder = instruction.type === "reorder-above" || instruction.type === "reorder-below";

  const getIndicatorStyles = () => ({
    position: "absolute" as const,
    height: isReorder ? "2px" : "100%",
    backgroundColor: isReorder ? "var(--moss-primary)" : "transparent",
    top: instruction.type === "reorder-above" ? (depth === 1 ? 0 : gap) : undefined,
    bottom: instruction.type === "reorder-below" ? gap : undefined,
    width: calculateWidth(),
    left: calculateLeft(),
  });

  const calculateWidth = () => {
    const baseWidth = `calc(100% - ${paddingRight}px - ${paddingLeft}px`;
    if (depth === 1) {
      return baseWidth + ")";
    }
    const offset = 16;
    return `${baseWidth} - ${offset}px)`;
  };

  const calculateLeft = () => {
    if (depth === 1) {
      return paddingLeft;
    }
    const folderOffset = isFolder ? (instruction.type === "reorder-above" ? 16 : 0) : 16;
    return paddingLeft + folderOffset;
  };

  const classNames = cn({
    "h-full w-full outline-2 -outline-offset-2 outline-[var(--moss-primary)]": instruction.type === "make-child",
  });

  return <div className={classNames} style={getIndicatorStyles()} {...props} />;
};
