import { useState } from "react";

import { useStreamCollections, useUpdateCollection } from "@/hooks";

export const useRenameCollectionForm = (collectionId: string) => {
  const [isRenamingCollection, setIsRenamingCollection] = useState(false);
  const { data: streamedCollections } = useStreamCollections();
  const collection = streamedCollections?.find((collection) => collection.id === collectionId);

  const { mutateAsync: updateCollection } = useUpdateCollection();

  const handleRenamingCollectionFormSubmit = async (newName: string) => {
    const trimmedNewName = newName.trim();

    try {
      if (trimmedNewName === collection?.name) {
        return;
      }

      await updateCollection({
        id: collectionId,
        name: trimmedNewName,
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
