import { CollectionListItem } from "./CollectionListItem/CollectionListItem";

export const CollectionListChildren = () => {
  return (
    <div>
      <CollectionListItem label="Dev" />
      <CollectionListItem label="Stage" />
      <CollectionListItem label="Prod" />
    </div>
  );
};
