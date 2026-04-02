import { useEffect } from "react";

import { resourceDetailsCollection } from "@/db/resourceDetails/resourceDetailsCollection";
import { ResourceDetails } from "@/db/resourceDetails/types";
import { resourceService } from "@/domains/resource/resourceService";

interface useSyncResourceDetailsProps {
  resourceId: string;
  projectId: string;
}

export const useSyncResourceDetails = ({ resourceId, projectId }: useSyncResourceDetailsProps) => {
  useEffect(() => {
    try {
      syncResourceDetails({ resourceId, projectId });
    } catch (error) {
      console.error("Error syncing resource details", error);
    }
  }, [projectId, resourceId]);
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
