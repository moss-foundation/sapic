import { useContext, useState } from "react";

import { useCreateCollectionEntry } from "@/hooks";

import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ProjectTreeRootNode } from "../../types";
import { createEntryKind } from "../../utils";

export const useRootNodeAddForm = (node: ProjectTreeRootNode) => {
  const { id } = useContext(ProjectTreeContext);

  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();

  const [isAddingRootFileNode, setIsAddingRootFileNode] = useState(false);
  const [isAddingRootFolderNode, setIsAddingRootFolderNode] = useState(false);

  const handleRootAddFormSubmit = async (name: string) => {
    const newName = name.trim();
    const newEntry = createEntryKind({
      name: newName,
      path: "",
      class: "Endpoint",
      isAddingFolder: isAddingRootFolderNode,
      order: node.childNodes.length + 1,
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
