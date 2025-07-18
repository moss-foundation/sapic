import { useContext, useState } from "react";

import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";
import { join, sep } from "@tauri-apps/api/path";

import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";

export const useNodeRenamingForm = (node: TreeCollectionNode, onNodeUpdate: (node: TreeCollectionNode) => void) => {
  const { id } = useContext(TreeContext);
  const [isRenamingNode, setIsRenamingNode] = useState(false);

  const { placeholderFnForUpdateCollectionEntry } = useUpdateCollectionEntry();

  const handleRenamingFormSubmit = async (newName: string) => {
    const rawpath = await join(...node.path.segments.slice(0, -1), newName);

    try {
      placeholderFnForUpdateCollectionEntry({
        collectionId: id,
        updatedEntry: {
          ...node,
          name: newName,
          path: {
            raw: rawpath,
            segments: rawpath.split(sep()),
          },
        },
      });

      onNodeUpdate({ ...node, name: newName });
    } catch (error) {
      console.error(error);
    } finally {
      setIsRenamingNode(false);
    }
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
