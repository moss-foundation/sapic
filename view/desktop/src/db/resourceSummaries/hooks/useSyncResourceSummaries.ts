import { useEffect, useEffectEvent, useMemo, useRef, useState } from "react";

import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
import { resourceService } from "@/domains/resource/resourceService";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";

export const useSyncResourceSummaries = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: localProjectSummaries, isLoading: isLocalProjectSummariesLoading } = useGetAllLocalProjectSummaries();
  const [isResourceSummariesLoading, setIsResourceSummariesLoading] = useState(false);
  const hasInitiallyLoaded = useRef(false);

  const isLoading = isLocalProjectSummariesLoading || isResourceSummariesLoading;

  const projectIdsKey = useMemo(
    () =>
      localProjectSummaries
        .map((p) => p.id)
        .sort()
        .join(","),
    [localProjectSummaries]
  );

  const updateResourceSummaries = useEffectEvent(async () => {
    if (!hasInitiallyLoaded.current) {
      setIsResourceSummariesLoading(true);
    }

    const syncedProjectIds = new Set(localProjectSummaries.map((p) => p.id));

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

    const allResourceIds = allResourcesWithProjectId.map(({ resource }) => resource.id);
    const fetchedResourceIds = new Set(allResourceIds);

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

    const staleIds: string[] = [];
    resourceSummariesCollection.forEach((resource) => {
      if (syncedProjectIds.has(resource.projectId) && !fetchedResourceIds.has(resource.id)) {
        staleIds.push(resource.id);
      }
    });
    staleIds.forEach((id) => resourceSummariesCollection.delete(id));

    setIsResourceSummariesLoading(false);
    hasInitiallyLoaded.current = true;
  });

  useEffect(() => {
    updateResourceSummaries();
  }, [projectIdsKey]);

  return { isLoading };
};
