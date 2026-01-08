import { useEffect } from "react";

import { useDescribeProjectResource } from "@/adapters/tanstackQuery/resource";
import { resourceDetailsCollection } from "@/db/resourceDetails/resourceDetailsCollection";

interface useSyncResourceDetailsProps {
  resourceId: string;
  projectId: string;
}

export const useSyncResourceDetails = ({ resourceId, projectId }: useSyncResourceDetailsProps) => {
  const { data: backendResourceDetails } = useDescribeProjectResource({
    projectId,
    resourceId,
  });

  useEffect(() => {
    if (!backendResourceDetails) return;

    const placeholderResourceDetails = {
      ...backendResourceDetails,
      id: resourceId,
      name: backendResourceDetails.name,
      url: backendResourceDetails.url,
      description: undefined,
      body: undefined,
      pathParams: backendResourceDetails.pathParams,
      queryParams: backendResourceDetails.queryParams,
      metadata: {
        isDirty: false,
      },
    };

    const hasResourceDetails = resourceDetailsCollection.has(resourceId);

    if (!hasResourceDetails) {
      resourceDetailsCollection.insert(placeholderResourceDetails);
    } else {
      resourceDetailsCollection.update(resourceId, (draft) => {
        if (!draft) return;

        Object.assign(draft, placeholderResourceDetails);
      });
    }
  }, [backendResourceDetails, resourceId]);
};
