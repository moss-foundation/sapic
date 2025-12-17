import { useEffect, useEffectEvent } from "react";

import { useDescribeProjectResource } from "@/adapters/tanstackQuery/project";
import { resourcesDescriptionsCollection } from "@/app/resourcesDescriptionsCollection";
import { DescribeResourceOutput } from "@repo/moss-project";
import { eq, useLiveQuery } from "@tanstack/react-db";

interface useSyncResourceDescriptionModelProps {
  resourceId: string;
  projectId: string;
}

export const useSyncResourceDescriptionModel = ({ resourceId, projectId }: useSyncResourceDescriptionModelProps) => {
  const { data: backendResourceDescription } = useDescribeProjectResource({ projectId, resourceId });

  const { data: localResourceDescription } = useLiveQuery((q) =>
    q
      .from({ collection: resourcesDescriptionsCollection })
      .where(({ collection }) => eq(collection.id, resourceId))
      .findOne()
  );

  useEffect(() => {
    if (!localResourceDescription && backendResourceDescription) {
      const placeholderResourceDescription = {
        ...backendResourceDescription,
        //FIXME this is a temporary solution because the backend returns hardcoded value for the url
        url:
          backendResourceDescription.url === "Hardcoded Value" || backendResourceDescription.url === undefined
            ? "{{baseUrl}}/docs/:docId/tables/:tableIdOrName/columns?sort={{sortValue}}&limit=2"
            : backendResourceDescription.url,
        description: undefined,
        body: undefined,
        //TODO: zod schema type should be updated.
        // Without this zod will throw and error because the descriptions that return from the backend are null
        // But in the schema we set them as optional and not as nullable
        //Error message:
        //SchemaValidationError: Insert validation failed:
        // - Expected string, received null - path: pathParams,0,description
        pathParams: backendResourceDescription.pathParams.map((param) => ({
          ...param,
          description: param.description ?? undefined,
        })),
        queryParams: backendResourceDescription.queryParams.map((param) => ({
          ...param,
          description: param.description ?? undefined,
        })),
      };

      resourcesDescriptionsCollection.insert({
        id: resourceId,
        ...placeholderResourceDescription,
      });
    }
  }, [backendResourceDescription, resourceId, localResourceDescription]);

  const updateLocalResourceDescription = useEffectEvent((backendResourceDescription: DescribeResourceOutput) => {
    if (!localResourceDescription || !backendResourceDescription) return;

    resourcesDescriptionsCollection.update(resourceId, (draft) => {
      if (!draft) return;

      const placeholderResourceDescription = {
        ...backendResourceDescription,
        id: resourceId,
        //FIXME this is a temporary solution because the backend returns hardcoded value for the url
        url:
          backendResourceDescription.url === "Hardcoded Value" || backendResourceDescription.url === undefined
            ? "{{baseUrl}}/docs/:docId/tables/:tableIdOrName/columns?sort={{sortValue}}&limit=2"
            : backendResourceDescription.url,
        description: undefined,
        body: undefined,
        //TODO: zod schema type should be updated.
        // Without this zod will throw and error because the descriptions that return from the backend are null
        // But in the schema we set them as optional and not as nullable
        //Error message:
        //SchemaValidationError: Insert validation failed:
        // - Expected string, received null - path: pathParams,0,description
        pathParams: backendResourceDescription.pathParams.map((param) => ({
          ...param,
          description: param.description ?? undefined,
        })),
        queryParams: backendResourceDescription.queryParams.map((param) => ({
          ...param,
          description: param.description ?? undefined,
        })),
      };

      Object.assign(draft, placeholderResourceDescription);
    });
  });

  useEffect(() => {
    if (!backendResourceDescription) return;
    updateLocalResourceDescription(backendResourceDescription);
  }, [backendResourceDescription]);
};
