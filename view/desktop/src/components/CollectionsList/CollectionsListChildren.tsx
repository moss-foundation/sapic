import { CollectionsListItem } from "./CollectionsListItem/CollectionsListItem";
import { CollectionWithEnvironment } from "./types";

export const CollectionsListChildren = ({ collection }: { collection: CollectionWithEnvironment }) => {
  return (
    <div>
      {collection.environments.map((environment) => (
        <CollectionsListItem key={environment.id} environment={environment} />
      ))}
    </div>
  );
};
