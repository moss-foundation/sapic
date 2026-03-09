import { useEffect } from "react";

import { projectSummariesCollection } from "@/db/projectSummaries/projectSummaries";
import { projectService } from "@/domains/project/projectService";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";

import { flushProjectSummaries } from "../actions/flushProjectSummaries";

export const useSyncProjectSummaries = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const refreshProjectSummaries = async ({ currentWorkspaceId }: { currentWorkspaceId: string }) => {
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

  useEffect(() => {
    refreshProjectSummaries({ currentWorkspaceId });
  }, [currentWorkspaceId]);

  return { isLoading: false, isPending: false, refreshProjectSummaries };
};
