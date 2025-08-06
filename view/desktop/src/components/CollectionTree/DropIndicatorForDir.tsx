import { HTMLAttributes } from "react";

import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

interface DropIndicatorForDirProps extends HTMLAttributes<HTMLDivElement> {
  isChildDropBlocked?: boolean | null;
  instruction: Instruction | null;
}

export const DropIndicatorForDir = ({ isChildDropBlocked, instruction, ...props }: DropIndicatorForDirProps) => {
  if (isChildDropBlocked) {
    return (
      <div
        {...props}
        style={{
          position: "absolute",
          height: "100%",
          width: "100%",
          top: 0,
          left: 0,
          backgroundColor: "var(--moss-error-background)",
        }}
      />
    );
  }

  if (!instruction) return null;

  if (instruction.operation === "combine") {
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
  }

  return null;
};
