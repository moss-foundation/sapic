import { useEffect, useEffectEvent } from "react";

import { useStreamedProjectsWithResources } from "@/adapters";
import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
import { useCurrentWorkspace } from "@/hooks";
import { useBatchGetTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useBatchGetTreeItemState";

export const useSyncResourceSummaries = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: projectsWithResources, isLoading } = useStreamedProjectsWithResources();

  const { data: treeItemStates } = useBatchGetTreeItemState(
    projectsWithResources.flatMap((project) => project.resources.map((resource) => resource.id)),
    currentWorkspaceId
  );

  const updateResourceSummaries = useEffectEvent(async () => {
    //remove all local resource summaries in case some resources are deleted from the project
    resourceSummariesCollection.forEach((resource) => {
      resourceSummariesCollection.delete(resource.id);
    });

    projectsWithResources.forEach((project) => {
      project.resources.forEach((resource) => {
        const hasResourceSummary = resourceSummariesCollection.has(resource.id);
        const treeItemState = treeItemStates?.find((treeItemState) => treeItemState.id === resource.id);

        if (hasResourceSummary) {
          resourceSummariesCollection.update(resource.id, (draft) => {
            Object.assign(draft, {
              ...resource,
              protocol: resource.protocol ?? undefined,

              order: treeItemState?.order ?? resource.order,
              expanded: treeItemState?.expanded ?? resource.expanded,
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

            order: treeItemState?.order ?? resource.order,
            expanded: treeItemState?.expanded ?? resource.expanded,
          });
        }
      });
    });
  });

  useEffect(() => {
    updateResourceSummaries();
  }, [projectsWithResources]);

  return { isLoading };
};
