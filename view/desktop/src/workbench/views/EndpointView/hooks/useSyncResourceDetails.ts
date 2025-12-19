import { useEffect } from "react";

import { useDescribeProjectResource } from "@/adapters/tanstackQuery/resource";
import { resourceDetailsCollection } from "@/db/resourceDetailsCollection";

interface useSyncResourceDetailsProps {
  resourceId: string;
  projectId: string;
}

export const useSyncResourceDetails = ({ resourceId, projectId }: useSyncResourceDetailsProps) => {
  const { data: backendResourceDetails } = useDescribeProjectResource({ projectId, resourceId });

  useEffect(() => {
    if (!backendResourceDetails) return;

    const placeholderResourceDetails = {
      ...backendResourceDetails,
      id: resourceId,
      //FIXME this is a temporary solution because the backend returns hardcoded value for the url
      url:
        backendResourceDetails.url === "Hardcoded Value" || backendResourceDetails.url === undefined
          ? "{{baseUrl}}/docs/:docId/tables/:tableIdOrName/columns?sort={{sortValue}}&limit=2"
          : backendResourceDetails.url,
      description: undefined,
      body: undefined,
      //TODO: zod schema type should be updated.
      // Without this zod will throw and error because the descriptions that return from the backend are null
      // But in the schema we set them as optional and not as nullable
      //Error message:
      //SchemaValidationError: Insert validation failed:
      // - Expected string, received null - path: pathParams,0,description
      pathParams: backendResourceDetails.pathParams.map((param) => ({
        ...param,
        description: param.description ?? undefined,
      })),
      queryParams: backendResourceDetails.queryParams.map((param) => ({
        ...param,
        description: param.description ?? undefined,
      })),
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
