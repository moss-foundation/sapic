import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

interface CombineDNDIndicatorProps {
  combineInstruction: Instruction | null | undefined;
}

export const CombineDNDIndicator = ({ combineInstruction }: CombineDNDIndicatorProps) => {
  if (!combineInstruction) return null;

  if (combineInstruction.operation === "combine") {
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
          backgroundColor: combineInstruction.blocked
            ? "var(--moss-error-background)"
            : "var(--moss-success-background)",
        }}
      />
    );
  }

  return null;
};
