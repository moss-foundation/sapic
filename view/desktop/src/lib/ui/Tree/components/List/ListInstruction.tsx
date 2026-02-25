import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

interface ListInstructionProps {
  instruction: Instruction | null;
}

export const ListInstruction = ({ instruction }: ListInstructionProps) => {
  if (!instruction || instruction.operation !== "combine") return null;

  const isBlocked = instruction.blocked;

  return (
    <div
      className={cn("absolute h-full w-full", {
        "background-(--moss-error-background)": isBlocked,
        "background-(--moss-success-background)": !isBlocked,
      })}
    />
  );
};
