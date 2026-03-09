import { DraggedResourceNode } from "@/workbench/ui/components/ProjectTree/types";

export const canCombineToProjectCreationZone = (sourceData: DraggedResourceNode | null) => {
  if (!sourceData) {
    return "not-available";
  }

  return "available";
};
