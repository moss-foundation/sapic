import { CollectionEnvironmentsListItem } from "./CollectionEnvironmentsListItem/CollectionEnvironmentsListItem";
import { CollectionWithEnvironment } from "./types";

interface CollectionEnvironmentsListChildrenProps {
  collectionsWithEnvironments: CollectionWithEnvironment;
}

export const CollectionEnvironmentsListChildren = ({
  collectionsWithEnvironments,
}: CollectionEnvironmentsListChildrenProps) => {
  return (
    <div>
      {collectionsWithEnvironments.environments.map((environment) => (
        <CollectionEnvironmentsListItem key={environment.id} environment={environment} />
      ))}
    </div>
  );
};
