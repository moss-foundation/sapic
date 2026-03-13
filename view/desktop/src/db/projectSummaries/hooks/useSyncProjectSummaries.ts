import { useEffect } from "react";

import { useListProjects } from "@/adapters/tanstackQuery/project";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";

import { flushProjectSummaries } from "../actions/flushProjectSummaries";
import { refreshProjectSummaries } from "../actions/refreshProjectSummaries";
import { projectSummariesCollection } from "../projectSummaries";

export const useSyncProjectSummaries = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: projects = { items: [] }, isPending: areProjectsPending } = useListProjects();

  useEffect(flushProjectSummaries, [currentWorkspaceId]);

  useEffect(() => {
    if (areProjectsPending || !projects) return;

    const syncProjectSummaries = async () => {
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
    };

    syncProjectSummaries();
  }, [areProjectsPending, currentWorkspaceId, projects]);

  return { isLoading: false, isPending: false, refreshProjectSummaries };
};
