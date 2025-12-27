import { eq, useLiveQuery } from "@tanstack/react-db";

import { resourceDetailsCollection } from "../resourceDetailsCollection";

export const useGetLocalResourceDetails = (resourceId: string) => {
  const { data: localResourceDetails } = useLiveQuery((q) =>
    q
      .from({ collection: resourceDetailsCollection })
      .where(({ collection }) => eq(collection.id, resourceId))
      .findOne()
  );

  return localResourceDetails;
};
