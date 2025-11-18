import { useContext } from "react";

import { USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, useStreamProjectResources } from "@/adapters/tanstackQuery/project";
import { useBatchUpdateProjectResource } from "@/adapters/tanstackQuery/project/useBatchUpdateProjectResource";
import { BatchUpdateResourceKind, StreamResourcesEvent } from "@repo/moss-project";
import { useQueryClient } from "@tanstack/react-query";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeRootNode } from "../types";

export const useToggleAllNodes = (node: ProjectTreeRootNode) => {
  const { id } = useContext(ProjectTreeContext);

  const queryClient = useQueryClient();

  const { data: streamedResources } = useStreamProjectResources(id);
  const { mutateAsync: batchUpdateProjectResource } = useBatchUpdateProjectResource();

  const expandAllNodes = async () => {
    if (!streamedResources) return;

    const resourcesToUpdate = streamedResources.filter((resource) => !resource.expanded && resource.kind === "Dir");

    const inputResources = resourcesToUpdate.map((resource): BatchUpdateResourceKind => {
      if (resource.kind === "Dir") {
        return {
          DIR: {
            id: resource.id,
            expanded: true,
          },
        };
      } else {
        return {
          ITEM: {
            id: resource.id,
            expanded: true,
            queryParamsToAdd: [],
            queryParamsToUpdate: [],
            queryParamsToRemove: [],
            pathParamsToAdd: [],
            pathParamsToUpdate: [],
            pathParamsToRemove: [],
            headersToAdd: [],
            headersToUpdate: [],
            headersToRemove: [],
          },
        };
      }
    });

    await batchUpdateProjectResource({
      projectId: node.id,
      resources: {
        resources: inputResources,
      },
    });

    queryClient.setQueryData([USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, id], (oldResources: StreamResourcesEvent[]) => {
      return oldResources.map((resource) => {
        if (resource.kind === "Dir" && !resource.expanded) {
          return { ...resource, expanded: true };
        }
        return resource;
      });
    });
  };

  const collapseAllNodes = async () => {
    if (!streamedResources) return;

    const resourcesToUpdate = streamedResources.filter((resource) => resource.expanded && resource.kind === "Dir");

    const inputResources = resourcesToUpdate.map((resource): BatchUpdateResourceKind => {
      if (resource.kind === "Dir") {
        return {
          DIR: {
            id: resource.id,
            expanded: false,
          },
        };
      } else {
        return {
          ITEM: {
            id: resource.id,
            expanded: false,
            queryParamsToAdd: [],
            queryParamsToUpdate: [],
            queryParamsToRemove: [],
            pathParamsToAdd: [],
            pathParamsToUpdate: [],
            pathParamsToRemove: [],
            headersToAdd: [],
            headersToUpdate: [],
            headersToRemove: [],
          },
        };
      }
    });

    await batchUpdateProjectResource({
      projectId: node.id,
      resources: {
        resources: inputResources,
      },
    });

    queryClient.setQueryData([USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, id], (oldResources: StreamResourcesEvent[]) => {
      return oldResources.map((resource) => {
        if (resource.kind === "Dir" && resource.expanded) {
          return { ...resource, expanded: false };
        }
        return resource;
      });
    });
  };

  return { expandAllNodes, collapseAllNodes };
};
