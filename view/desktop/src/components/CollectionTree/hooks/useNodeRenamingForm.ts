import { useContext, useState } from "react";

import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";
import { join } from "@tauri-apps/api/path";

import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";

export const useNodeRenamingForm = (node: TreeCollectionNode, onNodeUpdate: (node: TreeCollectionNode) => void) => {
  const { id } = useContext(TreeContext);
  const [isRenamingNode, setIsRenamingNode] = useState(false);

  const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry(id);

  const handleRenamingFormSubmit = async (newName: string) => {
    const rawpath = await join(...node.path.segments.slice(0, -1), newName);

    try {
      if (node.kind === "Dir") {
        await updateCollectionEntry({
          collectionId: id,
          updatedEntry: {
            "DIR": {
              id: node.id,
              path: rawpath,
              name: newName,
            },
          },
        });
      } else {
        await updateCollectionEntry({
          collectionId: id,
          updatedEntry: {
            "ITEM": {
              id: node.id,
              path: rawpath,
              name: newName,
            },
          },
        });
      }

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
