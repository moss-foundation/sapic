import { useContext, useState } from "react";

import { TreeContext } from "../../Tree";
import { TreeCollectionRootNode } from "../../types";

export const useRootNodeAddForm = (
  node: TreeCollectionRootNode,
  onNodeUpdateCallback: (node: TreeCollectionRootNode) => void
) => {
  const { sortBy } = useContext(TreeContext);

  const [isAddingRootNodeFile, setIsAddingRootNodeFile] = useState(false);
  const [isAddingRootNodeFolder, setIsAddingRootNodeFolder] = useState(false);

  const handleRootAddFormSubmit = (newNode: TreeCollectionRootNode) => {
    onNodeUpdateCallback({
      ...node,
      expanded: true,
      //   childNodes: [...node.childNodes, prepareCollectionForTree(newNode, sortBy, false)],
    });

    setIsAddingRootNodeFile(false);
    setIsAddingRootNodeFolder(false);
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
