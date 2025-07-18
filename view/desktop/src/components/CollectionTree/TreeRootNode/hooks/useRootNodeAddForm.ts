import { useContext, useState } from "react";

import { useCreateCollectionEntry } from "@/hooks";

import { TreeContext } from "../../Tree";
import { TreeCollectionRootNode } from "../../types";
import { createEntryKind } from "../../utils2";

export const useRootNodeAddForm = (node: TreeCollectionRootNode) => {
  const { id } = useContext(TreeContext);
  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();

  const [isAddingRootNodeFile, setIsAddingRootNodeFile] = useState(false);
  const [isAddingRootNodeFolder, setIsAddingRootNodeFolder] = useState(false);

  const handleRootAddFormSubmit = async (name: string) => {
    const newEntry = createEntryKind(
      name,
      "requests",
      isAddingRootNodeFolder,
      "Request",
      node.requests.childNodes.length + 1
    );

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
