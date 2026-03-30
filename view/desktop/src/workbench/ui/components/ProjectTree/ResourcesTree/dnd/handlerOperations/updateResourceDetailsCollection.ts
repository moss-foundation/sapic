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
    if (sourceResource?.collectionDetails) {
      resourceDetailsCollection.insert({ ...sourceResource.collectionDetails, id: newResource.id });
      resourceDetailsCollection.delete(sourceResource.id);
    }
  });
};
