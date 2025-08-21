import { useState } from "react";

import { useStreamedCollections, useUpdateCollection } from "@/hooks";

export const useRenameCollectionForm = (collectionId: string) => {
  const [isRenamingCollection, setIsRenamingCollection] = useState(false);
  const { data: streamedCollections } = useStreamedCollections();
  const collection = streamedCollections?.find((collection) => collection.id === collectionId);

  const { mutateAsync: updateCollection } = useUpdateCollection();

  const handleRenamingCollectionFormSubmit = async (name: string) => {
    try {
      if (name === collection?.name) {
        return;
      }

      await updateCollection({
        id: collectionId,
        name,
      });
    } catch (error) {
      console.error(error);
    } finally {
      setIsRenamingCollection(false);
    }
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
