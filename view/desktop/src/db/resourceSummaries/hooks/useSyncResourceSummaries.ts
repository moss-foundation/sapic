import { useEffect, useEffectEvent } from "react";

import { useProjectsWithResources } from "@/adapters";
import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/domains/treeItemState/service";

export const useSyncResourceSummaries = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: projectsWithResources, isLoading, isPending } = useProjectsWithResources();

  const updateResourceSummaries = useEffectEvent(async () => {
    //remove all local resource summaries in case some resources are deleted from the project
    resourceSummariesCollection.forEach((resource) => {
      resourceSummariesCollection.delete(resource.id);
    });

    const allResourceIds = projectsWithResources.flatMap((project) => project.resources.map((resource) => resource.id));

    const [treeItemOrders, treeItemExpanded] = await Promise.all([
      treeItemStateService.batchGetOrder(allResourceIds, currentWorkspaceId),
      treeItemStateService.batchGetExpanded(allResourceIds, currentWorkspaceId),
    ]);

    const orderMap = Object.fromEntries(allResourceIds.map((id, i) => [id, treeItemOrders[i]]));
    const expandedMap = Object.fromEntries(allResourceIds.map((id, i) => [id, treeItemExpanded[i]]));

    projectsWithResources.forEach((project) => {
      project.resources.forEach((resource) => {
        const hasResourceSummary = resourceSummariesCollection.has(resource.id);
        const order = orderMap[resource.id];
        const expanded = expandedMap[resource.id];

        if (hasResourceSummary) {
          resourceSummariesCollection.update(resource.id, (draft) => {
            Object.assign(draft, {
              ...resource,
              protocol: resource.protocol ?? undefined,

              order: order,
              expanded: expanded,
            });
          });
        } else {
          resourceSummariesCollection.insert({
            projectId: project.id,
            id: resource.id,
            name: resource.name,
            path: resource.path,
            class: resource.class,
            kind: resource.kind,
            protocol: resource.protocol ?? undefined,

            order: order,
            expanded: expanded,
          });
        }
      });
    });
  });

  useEffect(() => {
    updateResourceSummaries();
  }, [projectsWithResources]);

  return { isLoading, isPending };
};
