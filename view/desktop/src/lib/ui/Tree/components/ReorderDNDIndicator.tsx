import { CSSProperties } from "react";

import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

interface ReorderDNDIndicatorProps {
  reorderInstruction: Instruction | null;
  offsetLeft?: number;
}

export const ReorderDNDIndicator = ({ reorderInstruction, offsetLeft = 0 }: ReorderDNDIndicatorProps) => {
  if (!reorderInstruction || reorderInstruction.blocked || reorderInstruction.operation === "combine") return null;

  const styles: CSSProperties = {
    ...(reorderInstruction.operation === "reorder-before" ? { top: -0.5 } : { bottom: -0.5 }),
    width: `calc(100% - ${offsetLeft}px)`,
    pointerEvents: "none",
  } as const;

  return <div className="background-(--moss-accent) absolute h-[1px] w-full" style={styles} />;
};
