import { useContext, useState } from "react";

import { useCreateCollectionEntry } from "@/hooks";

import { CollectionTreeContext } from "../../CollectionTreeContext";
import { TreeCollectionRootNode } from "../../types";
import { createEntryKind } from "../../utils";

export const useRootNodeAddForm = (node: TreeCollectionRootNode) => {
  const { id } = useContext(CollectionTreeContext);

  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();

  const [isAddingRootNodeFile, setIsAddingRootNodeFile] = useState(false);
  const [isAddingRootNodeFolder, setIsAddingRootNodeFolder] = useState(false);

  const handleRootAddFormSubmit = async (name: string) => {
    const newEntry = createEntryKind({
      name: name.trim(),
      path: "requests",
      isAddingFolder: isAddingRootNodeFolder,
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
      setIsAddingRootNodeFile(false);
      setIsAddingRootNodeFolder(false);
    }
  };

  const handleRootAddFormCancel = () => {
    setIsAddingRootNodeFile(false);
    setIsAddingRootNodeFolder(false);
  };

  return {
    isAddingRootNodeFile,
    isAddingRootNodeFolder,
    setIsAddingRootNodeFile,
    setIsAddingRootNodeFolder,
    handleRootAddFormSubmit,
    handleRootAddFormCancel,
  };
};
