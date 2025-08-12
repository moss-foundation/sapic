import { CollectionsListRoot } from "./CollectionsListRoot/CollectionsListRoot";
import { useCollectionsWithEnvironments } from "./hooks/useCollectionsWithEnvironments";

export const CollectionsList = () => {
  const { collectionsWithEnvironments } = useCollectionsWithEnvironments();

  return (
    <div className="flex flex-col">
      {collectionsWithEnvironments?.map((collection) => (
        <CollectionsListRoot key={collection.id} collection={collection} />
      ))}
    </div>
  );
};
