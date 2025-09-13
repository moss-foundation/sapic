import { useContext, useState } from "react";

import { useFetchEntriesForPath } from "@/hooks/collection/derivedHooks/useFetchEntriesForPath";
import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";
import { join } from "@tauri-apps/api/path";

import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ProjectTreeNode } from "../../types";

export const useNodeRenamingForm = (node: ProjectTreeNode) => {
  const { id } = useContext(ProjectTreeContext);

  const { fetchEntriesForPath } = useFetchEntriesForPath();
  const [isRenamingNode, setIsRenamingNode] = useState(false);

  const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry();

  const handleRenamingFormSubmit = async (newName: string) => {
    const trimmedNewName = newName.trim();

    try {
      if (trimmedNewName === node.name) {
        return;
      }

      if (node.kind === "Dir") {
        await updateCollectionEntry({
          collectionId: id,
          updatedEntry: {
            DIR: {
              id: node.id,
              name: trimmedNewName,
            },
          },
        });

        const newPath = await join(...node.path.segments.slice(0, node.path.segments.length - 1), trimmedNewName);
        await fetchEntriesForPath(id, newPath);
      } else {
        await updateCollectionEntry({
          collectionId: id,
          updatedEntry: {
            ITEM: {
              id: node.id,
              name: trimmedNewName,
              queryParamsToAdd: [],
              queryParamsToUpdate: [],
              queryParamsToRemove: [],
              pathParamsToAdd: [],
              pathParamsToUpdate: [],
              pathParamsToRemove: [],
              headersToAdd: [],
              headersToUpdate: [],
              headersToRemove: [],
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
