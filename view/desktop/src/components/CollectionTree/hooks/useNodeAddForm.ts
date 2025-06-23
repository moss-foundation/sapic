import { useContext, useState } from "react";

import { useCollectionsStore } from "@/store/collections";
import { CreateEntryInput } from "@repo/moss-collection";

import { TreeContext } from "../Tree";

export const useNodeAddForm = () => {
  const { treeId } = useContext(TreeContext);
  const { createCollectionEntry } = useCollectionsStore();

  const [isAddingFileNode, setIsAddingFileNode] = useState(false);
  const [isAddingFolderNode, setIsAddingFolderNode] = useState(false);

  const handleAddFormSubmit = async (newEntry: CreateEntryInput) => {
    await createCollectionEntry({
      collectionId: treeId,
      input: newEntry,
    });

    setIsAddingFileNode(false);
    setIsAddingFolderNode(false);
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
