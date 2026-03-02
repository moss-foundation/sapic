import { forwardRef, HTMLAttributes, ReactNode } from "react";

import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

interface NodeProps extends HTMLAttributes<HTMLLIElement> {
  children: ReactNode;
  instruction?: Instruction | null;
  className?: string;
  isDragging?: boolean;
}

export const Node = forwardRef<HTMLLIElement, NodeProps>(
  ({ children, className, instruction, isDragging, ...props }: NodeProps, ref) => {
    return (
      <li ref={ref} className={cn("relative", className, { "opacity-50": isDragging })} {...props}>
        <TestDropIndicatorForDir instruction={instruction} />

        {children}
      </li>
    );
  }
);

//TODO remove this component
const TestDropIndicatorForDir = ({ instruction }: { instruction: Instruction | null | undefined }) => {
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
          zIndex: -1,
          pointerEvents: "none",
          backgroundColor: instruction.blocked ? "var(--moss-error-background)" : "var(--moss-success-background)",
        }}
      />
    );
  }

  return (
    <div
      style={{
        position: "absolute",
        height: "1px",
        width: "100%",
        top: instruction.operation === "reorder-before" ? 0 : "100%",
        left: 0,
        zIndex: -1,
        backgroundColor: "var(--moss-accent)",
        pointerEvents: "none",
      }}
    />
  );
};
