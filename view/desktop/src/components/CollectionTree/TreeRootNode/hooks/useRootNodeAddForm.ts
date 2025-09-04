import { useContext, useState } from "react";

import { useCreateCollectionEntry } from "@/hooks";

import { CollectionTreeContext } from "../../CollectionTreeContext";
import { TreeCollectionRootNode } from "../../types";
import { createEntryKind } from "../../utils";

export const useRootNodeAddForm = (node: TreeCollectionRootNode) => {
  const { id } = useContext(CollectionTreeContext);

  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();

  const [isAddingRootFileNode, setIsAddingRootFileNode] = useState(false);
  const [isAddingRootFolderNode, setIsAddingRootFolderNode] = useState(false);

  const handleRootAddFormSubmit = async (name: string) => {
    const newEntry = createEntryKind({
      name: name.trim(),
      path: "requests",
      isAddingFolder: isAddingRootFolderNode,
      order: node.requests.childNodes.length + 1,
      protocol: "Get",
    });

    try {
      await createCollectionEntry({
        collectionId: id,
        input: newEntry,
      });
    } catch (error) {
      console.error(error);
    } finally {
      setIsAddingRootFileNode(false);
      setIsAddingRootFolderNode(false);
    }
  };

  const handleRootAddFormCancel = () => {
    setIsAddingRootFileNode(false);
    setIsAddingRootFolderNode(false);
  };

  return {
    isAddingRootFileNode,
    isAddingRootFolderNode,
    setIsAddingRootFileNode,
    setIsAddingRootFolderNode,
    handleRootAddFormSubmit,
    handleRootAddFormCancel,
  };
};
