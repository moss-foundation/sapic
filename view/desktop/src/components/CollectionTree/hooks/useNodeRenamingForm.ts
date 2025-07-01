import { useContext, useState } from "react";

import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";

import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";

export const useNodeRenamingForm = (node: TreeCollectionNode, onNodeUpdate: (node: TreeCollectionNode) => void) => {
  const { id } = useContext(TreeContext);
  const [isRenamingNode, setIsRenamingNode] = useState(false);

  const { placeholderFnForUpdateCollectionEntry } = useUpdateCollectionEntry();

  const handleRenamingFormSubmit = (newName: string) => {
    onNodeUpdate({ ...node, name: newName });

    placeholderFnForUpdateCollectionEntry({
      id: node.id,
      collectionId: id,
      updatedEntry: { ...node, name: newName },
    });

    setIsRenamingNode(false);
  };

  const handleRenamingFormCancel = () => {
    setIsRenamingNode(false);
  };

  return {
    isRenamingNode,
    setIsRenamingNode,
    handleRenamingFormSubmit,
    handleRenamingFormCancel,
  };
};
