import { DragNode } from "@/workbench/ui/components/ProjectTree/types";

export const canCombineToProjectCreationZone = (sourceData: DragNode | null) => {
  if (!sourceData) {
    return "not-available";
  }

  return "available";
};
