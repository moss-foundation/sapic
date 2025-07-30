import { useContext, useState } from "react";

import { useCreateCollectionEntry } from "@/hooks";
import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";

import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";
import { createEntryKind } from "../utils";

export const useNodeAddForm = (parentNode: TreeCollectionNode) => {
  const { id } = useContext(TreeContext);
  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();
  const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry();

  const [isAddingFileNode, setIsAddingFileNode] = useState(false);
  const [isAddingFolderNode, setIsAddingFolderNode] = useState(false);

  const handleAddFormSubmit = async (name: string) => {
    const newEntry = createEntryKind(
      name.trim(),
      parentNode.path.raw,
      isAddingFolderNode,
      parentNode.class,
      parentNode.childNodes.length + 1
    );

    try {
      await createCollectionEntry({
        collectionId: id,
        input: newEntry,
      });

      await updateCollectionEntry({
        collectionId: id,
        updatedEntry: {
          DIR: {
            id: parentNode.id,
            expanded: true,
          },
        },
      });
    } catch (error) {
      console.error(error);
    } finally {
      setIsAddingFileNode(false);
      setIsAddingFolderNode(false);
    }
  };

  const handleAddFormCancel = () => {
    setIsAddingFileNode(false);
    setIsAddingFolderNode(false);
  };

  return {
    isAddingFileNode,
    isAddingFolderNode,
    setIsAddingFileNode,
    setIsAddingFolderNode,
    handleAddFormSubmit,
    handleAddFormCancel,
  };
};
