import { resourceDetailsCollection } from "@/db/resourceDetails/resourceDetailsCollection";
import { BatchCreateResourceOutput } from "@repo/moss-project";

import { ResourceNodeWithDetails } from "../types.dnd";

export const updateResourceDetailsCollection = ({
  allFlatSourceResourceNodes,
  batchCreateResourceOutput,
}: {
  allFlatSourceResourceNodes: ResourceNodeWithDetails[];
  batchCreateResourceOutput: BatchCreateResourceOutput;
}) => {
  batchCreateResourceOutput.resources.forEach((newResource, index) => {
    const sourceResource = allFlatSourceResourceNodes[index];

    resourceDetailsCollection.insert({
      ...sourceResource.details,
      id: newResource.id,
      metadata: { isDirty: false },
      protocol: sourceResource.details.protocol ?? undefined,
      url: sourceResource.details.url ?? undefined,
      queryParams: sourceResource.details.queryParams.map((qp) => ({
        ...qp,
        description: qp.description ?? undefined,
      })),
      body: sourceResource.details.body ?? undefined,
    });

    if (sourceResource?.collectionDetails && sourceResource?.collectionDetails.metadata.isDirty) {
      const {
        id: _sourceId,
        queryParams,
        headers,
        pathParams,
        ...restCollectionDetails
      } = sourceResource.collectionDetails;

      resourceDetailsCollection.update(newResource.id, (draft) => {
        Object.assign(draft, {
          ...restCollectionDetails,
          body: restCollectionDetails.body ?? undefined,
        });

        if (queryParams) {
          draft.queryParams = queryParams.map((sourceQp, i) => {
            const draftQp = draft.queryParams[i];
            return {
              ...(draftQp ?? {}),
              ...sourceQp,
              id: draftQp?.id ?? sourceQp.id,
              description: sourceQp.description ?? undefined,
            };
          });
        }
        if (headers) {
          draft.headers = headers.map((sourceH, i) => {
            const draftH = draft.headers[i];
            return {
              ...(draftH ?? {}),
              ...sourceH,
              id: draftH?.id ?? sourceH.id,
              description: sourceH.description ?? undefined,
            };
          });
        }
        if (pathParams) {
          draft.pathParams = pathParams.map((sourceP, i) => {
            const draftP = draft.pathParams[i];
            return {
              ...(draftP ?? {}),
              ...sourceP,
              id: draftP?.id ?? sourceP.id,
              description: sourceP.description ?? undefined,
            };
          });
        }
      });

      resourceDetailsCollection.delete(sourceResource.id);
    }
  });
};
