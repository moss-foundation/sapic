import { CollectionEnvironmentsListRoot } from "./CollectionEnvironmentsListRoot/CollectionEnvironmentsListRoot";
import { useCollectionsWithEnvironments } from "./hooks/useCollectionsWithEnvironments";

export const CollectionEnvironmentsList = () => {
  const { collectionsWithEnvironments } = useCollectionsWithEnvironments();

  return (
    <div className="flex flex-col">
      {collectionsWithEnvironments?.map((collectionsWithEnvironments) => (
        <CollectionEnvironmentsListRoot
          key={collectionsWithEnvironments.id}
          collectionsWithEnvironments={collectionsWithEnvironments}
        />
      ))}
    </div>
  );
};
