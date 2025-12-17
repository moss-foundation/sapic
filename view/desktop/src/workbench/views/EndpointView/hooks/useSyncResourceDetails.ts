import { useEffect, useEffectEvent } from "react";

import { useDescribeProjectResource } from "@/adapters/tanstackQuery/resource";
import { resourceDetailsCollection } from "@/db/resourceSummariesCollection";
import { DescribeResourceOutput } from "@repo/moss-project";
import { eq, useLiveQuery } from "@tanstack/react-db";

interface useSyncResourceDetailsProps {
  resourceId: string;
  projectId: string;
}

export const useSyncResourceDetails = ({ resourceId, projectId }: useSyncResourceDetailsProps) => {
  const { data: backendResourceDetails } = useDescribeProjectResource({ projectId, resourceId });

  const { data: localResourceDetails } = useLiveQuery((q) =>
    q
      .from({ collection: resourceDetailsCollection })
      .where(({ collection }) => eq(collection.id, resourceId))
      .findOne()
  );

  useEffect(() => {
    if (!localResourceDetails && backendResourceDetails) {
      const placeholderResourceDetails = {
        ...backendResourceDetails,
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

      resourceDetailsCollection.insert({
        id: resourceId,
        ...placeholderResourceDetails,
      });
    }
  }, [backendResourceDetails, resourceId, localResourceDetails]);

  const updateLocalResourceDetails = useEffectEvent((backendResourceDetails: DescribeResourceOutput) => {
    if (!localResourceDetails || !backendResourceDetails) return;

    resourceDetailsCollection.update(resourceId, (draft) => {
      if (!draft) return;

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

      Object.assign(draft, placeholderResourceDetails);
    });
  });

  useEffect(() => {
    if (!backendResourceDetails) return;
    updateLocalResourceDetails(backendResourceDetails);
  }, [backendResourceDetails]);
};
