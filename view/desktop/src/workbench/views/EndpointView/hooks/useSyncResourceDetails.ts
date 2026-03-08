import { useEffect } from "react";

import { resourceDetailsCollection } from "@/db/resourceDetails/resourceDetailsCollection";
import { resourceService } from "@/domains/resource/resourceService";

interface useSyncResourceDetailsProps {
  resourceId: string;
  projectId: string;
}

export const useSyncResourceDetails = ({ resourceId, projectId }: useSyncResourceDetailsProps) => {
  useEffect(() => {
    resourceService.describe(projectId, resourceId).then((resource) => {
      if (resourceDetailsCollection.has(resourceId)) {
        resourceDetailsCollection.update(resourceId, (draft) => {
          Object.assign(draft, resource);
        });
      } else {
        resourceDetailsCollection.insert({
          ...resource,
          id: resourceId,
          metadata: {
            isDirty: false,
          },
        });
      }
    });
  }, [projectId, resourceId]);
};
