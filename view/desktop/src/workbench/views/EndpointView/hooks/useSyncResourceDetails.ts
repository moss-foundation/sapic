import { useEffect } from "react";

import { USE_LIST_PROJECTS_QUERY_KEY } from "@/adapters/tanstackQuery/project";
import { USE_LIST_PROJECT_RESOURCES_QUERY_KEY } from "@/adapters/tanstackQuery/resource";
import { resourceDetailsCollection } from "@/db/resourceDetails/resourceDetailsCollection";
import { ResourceDetails } from "@/db/resourceDetails/types";
import { resourceService } from "@/domains/resource/resourceService";
import { useIsFetching } from "@tanstack/react-query";

interface useSyncResourceDetailsProps {
  resourceId: string;
  projectId: string;
}

export const useSyncResourceDetails = ({ resourceId, projectId }: useSyncResourceDetailsProps) => {
  const { isLoading: isProjectsViewLoading } = useProjectsViewFetchingTracking(projectId);

  useEffect(() => {
    if (isProjectsViewLoading) return;
    try {
      syncResourceDetails({ resourceId, projectId });
    } catch (error) {
      console.error("Error syncing resource details", error);
    }
  }, [projectId, resourceId, isProjectsViewLoading]);

  return {
    isSyncing: isProjectsViewLoading,
  };
};

const syncResourceDetails = async ({ resourceId, projectId }: { resourceId: string; projectId: string }) => {
  const resourceDetails = await resourceService.describe(projectId, resourceId);
  if (!resourceDetails) return;

  const sanitized = {
    ...resourceDetails,
    protocol: resourceDetails.protocol ?? undefined,
    url: resourceDetails.url ?? undefined,
    body: resourceDetails.body ?? undefined,
    queryParams: resourceDetails.queryParams?.map((p) => ({
      ...p,
      description: p.description ?? undefined,
    })),
    headers: resourceDetails.headers?.map((h) => ({
      ...h,
      description: h.description ?? undefined,
    })),
    pathParams: resourceDetails.pathParams?.map((p) => ({
      ...p,
      description: p.description ?? undefined,
    })),
  } satisfies Omit<ResourceDetails, "id" | "metadata">;

  if (resourceDetailsCollection.has(resourceId)) {
    const existing = resourceDetailsCollection.get(resourceId);
    if (existing?.metadata?.isDirty) {
      return;
    }

    resourceDetailsCollection.update(resourceId, (draft) => {
      Object.assign(draft, {
        ...sanitized,
        metadata: {
          isDirty: false,
        },
      });
    });
  } else {
    console.log("inserting resource details", resourceId);
    resourceDetailsCollection.insert({
      ...sanitized,
      id: resourceId,
      metadata: {
        isDirty: false,
      },
    });
  }
};

const useProjectsViewFetchingTracking = (projectId: string) => {
  const projectsFetchingCount = useIsFetching({
    queryKey: [USE_LIST_PROJECTS_QUERY_KEY],
  });
  const projectResourcesFetchingCount = useIsFetching({
    queryKey: [USE_LIST_PROJECT_RESOURCES_QUERY_KEY, projectId],
  });

  const isLoading = projectsFetchingCount > 0 || projectResourcesFetchingCount > 0;

  return {
    isLoading,
  };
};
