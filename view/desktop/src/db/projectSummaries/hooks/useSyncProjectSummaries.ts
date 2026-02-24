import { useEffect, useEffectEvent } from "react";

import { useListProjects } from "@/adapters/tanstackQuery/project/useListProjects";
import { projectSummariesCollection } from "@/db/projectSummaries/projectSummaries";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { ListProjectItem } from "@repo/ipc";

import { flushProjectSummaries } from "../actions/flushProjectSummaries";

export const useSyncProjectSummaries = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: projects, isLoading, isPending } = useListProjects();

  const updateProjectSummaries = useEffectEvent(async (projectItems: ListProjectItem[]) => {
    if (projectItems.length === 0) {
      flushProjectSummaries();
      return;
    }

    const treeItemOrders = await treeItemStateService.batchGetOrder(
      projectItems.map((project) => project.id),
      currentWorkspaceId
    );
    const treeItemExpanded = await treeItemStateService.batchGetExpanded(
      projectItems.map((project) => project.id),
      currentWorkspaceId
    );

    projectItems.forEach((project, index) => {
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
      const doesProjectExistInRemote = projectItems.some((p) => p.id === project.id);
      if (!doesProjectExistInRemote) projectSummariesCollection.delete(project.id);
    });
  });

  useEffect(flushProjectSummaries, [currentWorkspaceId]);

  useEffect(() => {
    if (!projects) return;
    updateProjectSummaries(projects.items);
  }, [projects]);

  return { isLoading, isPending };
};
