/* eslint-disable mossLint/only-valid-token-names */
import type { CSSProperties, HTMLAttributes } from "react";

import { cn } from "@/utils";
import type { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/types";

type Orientation = "horizontal" | "vertical";

const edgeToOrientationMap: Record<Edge, Orientation> = {
  top: "horizontal",
  bottom: "horizontal",
  left: "vertical",
  right: "vertical",
};

const orientationStyles: Record<Orientation, HTMLAttributes<HTMLElement>["className"]> = {
  horizontal: "h-(--line-thickness) left-(--terminal-radius) right-0 before:left-(--negative-terminal-size)",
  vertical: "w-(--line-thickness) top-(--terminal-radius) bottom-0 before:top-(--negative-terminal-size)",
};

const edgeStyles: Record<Edge, HTMLAttributes<HTMLElement>["className"]> = {
  top: "top-(--line-offset) before:top-(--offset-terminal)",
  right: "right-(--line-offset) before:right-(--offset-terminal)",
  bottom: "bottom-(--line-offset) before:bottom-(--offset-terminal)",
  left: "left-(--line-offset) before:left-(--offset-terminal)",
};

export interface DropIndicatorProps {
  edge: Edge;
  gap?: number;
  strokeSize?: number;
  terminalSize?: number;
  className?: string;
  noTerminal?: boolean;
}

export function DropIndicator({
  edge,
  noTerminal = false,
  gap = 0,
  strokeSize = 2,
  terminalSize = 8,
  className,
}: DropIndicatorProps) {
  const lineOffset = -0.5 * (gap + strokeSize);
  const offsetToAlignTerminalWithLine = (strokeSize - terminalSize) / 2;

  const orientation = edgeToOrientationMap[edge];

  return (
    <div
      style={
        {
          "--line-thickness": `${strokeSize}px`,
          "--line-offset": `${lineOffset}px`,
          "--terminal-size": `${terminalSize}px`,
          "--terminal-radius": noTerminal ? `0px` : `${terminalSize / 2}px`,
          "--negative-terminal-size": `-${terminalSize}px`,
          "--offset-terminal": `${offsetToAlignTerminalWithLine}px`,
        } as CSSProperties
      }
      className={cn(
        `background-current pointer-events-none absolute z-10 box-border`,
        {
          "before:absolute before:h-(--terminal-size) before:w-(--terminal-size) before:rounded-full before:border-(length:--line-thickness) before:border-solid before:border-current before:content-['']":
            noTerminal === false,
        },
        orientationStyles[orientation],
        edgeStyles[edge],
        className
      )}
    />
  );
}

export default DropIndicator;
