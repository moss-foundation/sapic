import { useContext, useState } from "react";

import { TreeContext } from "../Tree";
import { NodeProps, TreeNodeProps } from "../types";
import { prepareCollectionForTree } from "../utils";

export const useNodeAddForm = (node: TreeNodeProps, onNodeUpdateCallback: (node: TreeNodeProps) => void) => {
  const { sortBy } = useContext(TreeContext);

  const [isAddingFileNode, setIsAddingFileNode] = useState(false);
  const [isAddingFolderNode, setIsAddingFolderNode] = useState(false);

  const handleAddFormSubmit = (newNode: NodeProps) => {
    onNodeUpdateCallback({
      ...node,
      isExpanded: true,
      childNodes: [...node.childNodes, prepareCollectionForTree(newNode, sortBy, false)],
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
