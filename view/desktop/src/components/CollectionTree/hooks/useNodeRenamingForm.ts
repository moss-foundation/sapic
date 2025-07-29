import { useContext, useState } from "react";

import { useFetchEntriesForPath } from "@/hooks/collection/derivedHooks/useFetchEntriesForPath";
import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";
import { join } from "@tauri-apps/api/path";

import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";

export const useNodeRenamingForm = (node: TreeCollectionNode) => {
  const { id } = useContext(TreeContext);
  const { fetchEntriesForPath } = useFetchEntriesForPath();
  const [isRenamingNode, setIsRenamingNode] = useState(false);

  const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry();

  const handleRenamingFormSubmit = async (newName: string) => {
    try {
      if (node.kind === "Dir") {
        await updateCollectionEntry({
          collectionId: id,
          updatedEntry: {
            DIR: {
              id: node.id,
              name: newName,
            },
          },
        });

        const newPath = await join(...node.path.segments.slice(0, node.path.segments.length - 1), newName);
        await fetchEntriesForPath(id, newPath);
      } else {
        await updateCollectionEntry({
          collectionId: id,
          updatedEntry: {
            ITEM: {
              id: node.id,
              name: newName,
            },
          },
        });
      }
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
