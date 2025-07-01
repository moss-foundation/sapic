import { useContext, useState } from "react";

import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";

import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";

export const useNodeRenamingForm = (node: TreeCollectionNode, onNodeUpdate: (node: TreeCollectionNode) => void) => {
  const { id } = useContext(TreeContext);
  const [isRenamingNode, setIsRenamingNode] = useState(false);

  const { placeholderFnForUpdateCollectionEntry } = useUpdateCollectionEntry();

  const handleRenamingFormSubmit = async (newName: string) => {
    onNodeUpdate({ ...node, name: newName });

    // const rawpath = await join(...node.path.segments.slice(0, -1), newName);

    // placeholderFnForUpdateCollectionEntry({
    //   collectionId: id,
    //   updatedEntry: {
    //     ...node,
    //     name: newName,
    //     path: {
    //       raw: rawpath,
    //       segments: rawpath.split(sep()),
    //     },
    //   },
    // });

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
