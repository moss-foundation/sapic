import { useEffect, useRef } from "react";

import { useListProjects } from "@/adapters/tanstackQuery/project/useListProjects";
import { projectSummariesCollection } from "@/db/projectSummaries/projectSummaries";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/domains/treeItemState/service";

export const useSyncProjectSummaries = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const hasSyncedRef = useRef(false);
  const lastWorkspaceIdRef = useRef<string | undefined>(currentWorkspaceId);

  const { data: projects, isLoading, isPending } = useListProjects();

  // Reset sync flag when workspace changes
  useEffect(() => {
    if (lastWorkspaceIdRef.current !== currentWorkspaceId) {
      lastWorkspaceIdRef.current = currentWorkspaceId;
      hasSyncedRef.current = false;

      projectSummariesCollection.forEach((project) => {
        projectSummariesCollection.delete(project.id);
      });
    }
  }, [currentWorkspaceId]);

  useEffect(() => {
    // Only sync on initial load when data is available
    if (hasSyncedRef.current || !projects || projects.items.length === 0) {
      return;
    }

    const updateLocalProjects = async () => {
      const treeItemOrders = await treeItemStateService.batchGetOrder(
        projects.items.map((project) => project.id),
        currentWorkspaceId
      );
      const treeItemExpanded = await treeItemStateService.batchGetExpanded(
        projects.items.map((project) => project.id),
        currentWorkspaceId
      );

      for (const project of projects.items) {
        const order = treeItemOrders?.[project.id];
        const expanded = treeItemExpanded?.[project.id];

        if (projectSummariesCollection.has(project.id)) {
          projectSummariesCollection.update(project.id, (draft) => {
            draft.order = order ?? -1;
            draft.expanded = expanded ?? true;
          });
        } else {
          projectSummariesCollection.insert({
            ...project,
            order: order ?? -1,
            expanded: expanded ?? true,
          });
        }
      }
      hasSyncedRef.current = true;
    };

    updateLocalProjects();
  }, [currentWorkspaceId, projects]);

  return { isLoading, isPending };
};
