import { extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

export const getInstructionFromFirstLocation = (location: DragLocationHistory): Instruction | null => {
  if (location.current.dropTargets.length === 0) return null;
  return extractInstruction(location.current.dropTargets[0].data);
};
