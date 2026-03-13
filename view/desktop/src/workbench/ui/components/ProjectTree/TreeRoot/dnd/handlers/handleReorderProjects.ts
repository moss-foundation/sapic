import { projectSummariesCollection } from "@/db/projectSummaries/projectSummaries";
import { filterUpdatedOrders } from "@/utils/filterUpdatedOrders";
import { moveItemInArray } from "@/utils/moveItemInArray";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { extractInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { getTreeRootLocationData } from "../getters/getTreeRootLocationData";
import { getTreeRootSourceData } from "../getters/getTreeRootSourceData";

interface HandleReorderProjectsProps {
  location: DragLocationHistory;
  source: ElementDragPayload;
  currentWorkspaceId: string;
}

export const handleReorderProjects = async ({ location, source, currentWorkspaceId }: HandleReorderProjectsProps) => {
  if (location.current?.dropTargets.length === 0) return;

  const sourceData = getTreeRootSourceData(source);
  const locationData = getTreeRootLocationData(location);
  const instruction = extractInstruction(locationData);

  if (locationData.data.projectId === sourceData.data.projectId) {
    return;
  }

  try {
    const sorted = sortObjectsByOrder(projectSummariesCollection.map((p) => p));
    const sourceIndex = sorted.findIndex((p) => p.id === sourceData.data.projectId);
    const locationIndex = sorted.findIndex((p) => p.id === locationData.data.projectId);

    if (sourceIndex === -1 || locationIndex === -1) {
      console.error("Source or location project not found", { sourceData, locationData });
      return;
    }

    const insertAtIndex = instruction?.operation === "reorder-before" ? locationIndex : locationIndex + 1;
    const projectToMove = sourceData.data.node;

    const inserted = moveItemInArray({ arr: sorted, itemToMove: projectToMove, toIndex: insertAtIndex });
    const projectsToUpdate = filterUpdatedOrders(sorted, inserted);

    await treeItemStateService.batchPutOrder(projectsToUpdate, currentWorkspaceId);
  } catch (error) {
    console.error("Error reordering projects:", error);
  }
};
