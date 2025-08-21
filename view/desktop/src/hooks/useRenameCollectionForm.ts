import { useState } from "react";

import { useUpdateCollection } from "@/hooks";

export const useRenameCollectionForm = (collectionId: string) => {
  const [isRenamingCollection, setIsRenamingCollection] = useState(false);

  const { mutateAsync: updateCollection } = useUpdateCollection();

  const handleRenamingCollectionFormSubmit = (name: string) => {
    updateCollection({
      id: collectionId,
      name,
    });

    setIsRenamingCollection(false);
  };

  const handleRenamingCollectionFormCancel = () => {
    setIsRenamingCollection(false);
  };

  return {
    isRenamingCollection,
    setIsRenamingCollection,
    handleRenamingCollectionFormSubmit,
    handleRenamingCollectionFormCancel,
  };
};
