import { useState } from "react";

import { useStreamedCollections } from "@/hooks/collection";

import { CollectionListChildren } from "./CollectionListChildren";
import { CollectionListHeader } from "./CollectionListHeader/CollectionListHeader";

interface CollectionListProps {
  id: string;
}

export const CollectionList = ({ id }: CollectionListProps) => {
  const { data: collections } = useStreamedCollections();

  const collection = collections?.find((collection) => collection.id === id);

  const [showChildren, setShowChildren] = useState(true);

  if (!collection) {
    return null;
  }

  return (
    <div>
      <CollectionListHeader collection={collection} onToggleChildren={setShowChildren} showChildren={showChildren} />
      {showChildren && <CollectionListChildren />}
    </div>
  );
};
