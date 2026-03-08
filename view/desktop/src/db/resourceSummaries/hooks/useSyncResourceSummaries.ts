import { useEffect, useEffectEvent, useState } from "react";

import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
import { resourceService } from "@/domains/resource/resourceService";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";

export const useSyncResourceSummaries = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: localProjectSummaries, isLoading: isLocalProjectSummariesLoading } = useGetAllLocalProjectSummaries();
  const [isResourceSummariesLoading, setIsResourceSummariesLoading] = useState(false);

  const isLoading = isLocalProjectSummariesLoading || isResourceSummariesLoading;

  const updateResourceSummaries = useEffectEvent(async () => {
    setIsResourceSummariesLoading(true);
    //remove all local resource summaries in case some resources are deleted from the project
    resourceSummariesCollection.forEach((resource) => {
      resourceSummariesCollection.delete(resource.id);
    });

    const allResourcesWithProjectId = (
      await Promise.all(
        localProjectSummaries.map(async (project) => {
          const resources = await resourceService.list({ projectId: project.id, mode: "LOAD_ROOT" });
          return resources.items.map((resource) => ({
            projectId: project.id,
            resource,
          }));
        })
      )
    ).flatMap((items) => items);
    console.log({ allResourcesWithProjectId });

    const allResourceIds = allResourcesWithProjectId.map(({ resource }) => resource.id);

    const [treeItemOrders, treeItemExpanded] = await Promise.all([
      treeItemStateService.batchGetOrder(allResourceIds, currentWorkspaceId),
      treeItemStateService.batchGetExpanded(allResourceIds, currentWorkspaceId),
    ]);

    const orderMap = Object.fromEntries(allResourceIds.map((id, i) => [id, treeItemOrders[i]]));
    const expandedMap = Object.fromEntries(allResourceIds.map((id, i) => [id, treeItemExpanded[i]]));

    allResourcesWithProjectId.forEach(({ resource, projectId }) => {
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
          projectId,
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

    setIsResourceSummariesLoading(false);
  });

  useEffect(() => {
    updateResourceSummaries();
  }, [localProjectSummaries]);

  return { isLoading };
};
