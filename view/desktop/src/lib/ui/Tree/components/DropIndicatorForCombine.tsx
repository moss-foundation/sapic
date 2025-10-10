import { HTMLAttributes } from "react";

import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

interface DropIndicatorForCombineProps extends HTMLAttributes<HTMLDivElement> {
  instruction: Instruction | null;
}

export const DropIndicatorForCombine = ({ instruction, ...props }: DropIndicatorForCombineProps) => {
  if (!instruction) return null;

  return (
    <div
      style={{
        position: "absolute",
        height: "100%",
        width: "100%",
        top: 0,
        left: 0,
        backgroundColor: instruction.blocked ? "var(--moss-error-background)" : "var(--moss-success-background)",
      }}
      {...props}
    />
  );
};
