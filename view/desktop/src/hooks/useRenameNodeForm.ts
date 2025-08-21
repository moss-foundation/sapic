import { useState } from "react";

import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { useFetchEntriesForPath } from "@/hooks/collection/derivedHooks/useFetchEntriesForPath";
import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { join } from "@tauri-apps/api/path";

export const useRenameNodeForm = (node: TreeCollectionNode, collectionId: string) => {
  const { fetchEntriesForPath } = useFetchEntriesForPath();
  const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry();

  const { api } = useTabbedPaneStore();

  const [isRenamingNode, setIsRenamingNode] = useState(false);

  const handleRenamingFormSubmit = async (newName: string) => {
    try {
      if (node.kind === "Dir") {
        await updateCollectionEntry({
          collectionId,
          updatedEntry: {
            DIR: {
              id: node.id,
              name: newName,
            },
          },
        });

        const newPath = await join(...node.path.segments.slice(0, node.path.segments.length - 1), newName);
        await fetchEntriesForPath(collectionId, newPath);
      } else {
        await updateCollectionEntry({
          collectionId,
          updatedEntry: {
            ITEM: {
              id: node.id,
              name: newName,
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

      const panel = api?.getPanel(node.id);
      if (panel) {
        panel.setTitle(newName);
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
