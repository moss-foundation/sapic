import { useState } from "react";

import { NodeProps, TreeNodeProps } from "../types";
import { prepareCollectionForTree, sortNodes } from "../utils";

export const useNodeAddForm = (node: TreeNodeProps, onNodeUpdate: (node: TreeNodeProps) => void) => {
  const [isAddingFileNode, setIsAddingFileNode] = useState(false);
  const [isAddingFolderNode, setIsAddingFolderNode] = useState(false);

  const handleAddFormSubmit = (newNode: NodeProps) => {
    onNodeUpdate({
      ...node,
      isExpanded: true,
      childNodes: sortNodes([...node.childNodes, prepareCollectionForTree(newNode, false)]),
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
