import { DragResourceNodeData } from "@/workbench/ui/components/ProjectTree/ResourcesTree/dnd/types.dnd";

export const canCombineToProjectCreationZone = (sourceData: DragResourceNodeData | null) => {
  if (!sourceData) {
    return "not-available";
  }

  return "available";
};
