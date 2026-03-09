import { projectService } from "@/domains/project/projectService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";

import { projectSummariesCollection } from "../projectSummaries";
import { flushProjectSummaries } from "./flushProjectSummaries";

export const refreshProjectSummaries = async ({ currentWorkspaceId }: { currentWorkspaceId: string }) => {
  flushProjectSummaries();

  const projects = await projectService.list();

  const treeItemOrders = await treeItemStateService.batchGetOrder(
    projects.items.map((project) => project.id),
    currentWorkspaceId
  );
  const treeItemExpanded = await treeItemStateService.batchGetExpanded(
    projects.items.map((project) => project.id),
    currentWorkspaceId
  );

  projects.items.forEach((project, index) => {
    const order = treeItemOrders?.[index];
    const expanded = treeItemExpanded?.[index];

    if (projectSummariesCollection.has(project.id)) {
      projectSummariesCollection.update(project.id, (draft) => {
        draft.order = order;
        draft.expanded = expanded;
      });
    } else {
      projectSummariesCollection.insert({
        ...project,
        order,
        expanded,
      });
    }
  });

  projectSummariesCollection.forEach((project) => {
    const doesProjectExistInRemote = projects.items.some((p) => p.id === project.id);
    if (!doesProjectExistInRemote) projectSummariesCollection.delete(project.id);
  });
};
