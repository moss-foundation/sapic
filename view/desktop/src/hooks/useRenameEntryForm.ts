import { useState } from "react";

import { useFetchEntriesForPath } from "@/hooks/collection/derivedHooks/useFetchEntriesForPath";
import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { EntryInfo } from "@repo/moss-collection";
import { join } from "@tauri-apps/api/path";

export const useRenameEntryForm = (entry: EntryInfo, collectionId: string) => {
  const { fetchEntriesForPath } = useFetchEntriesForPath();
  const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry();
  const { api } = useTabbedPaneStore();

  const [isRenamingEntry, setIsRenamingEntry] = useState(false);

  const handleRenamingEntrySubmit = async (newName: string) => {
    try {
      const trimmedNewName = newName.trim();

      if (trimmedNewName === entry.name) {
        return;
      }

      if (entry.kind === "Dir") {
        await updateCollectionEntry({
          collectionId,
          updatedEntry: {
            DIR: {
              id: entry.id,
              name: trimmedNewName,
            },
          },
        });

        const newPath = await join(...entry.path.segments.slice(0, entry.path.segments.length - 1), trimmedNewName);
        await fetchEntriesForPath(collectionId, newPath);
      } else {
        await updateCollectionEntry({
          collectionId,
          updatedEntry: {
            ITEM: {
              id: entry.id,
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
      const panel = api?.getPanel(entry.id);
      if (panel) {
        panel.setTitle(trimmedNewName);
      }
    } catch (error) {
      console.error(error);
    } finally {
      setIsRenamingEntry(false);
    }
  };

  const handleRenamingEntryCancel = () => {
    setIsRenamingEntry(false);
  };

  return {
    isRenamingEntry,
    setIsRenamingEntry,
    handleRenamingEntrySubmit,
    handleRenamingEntryCancel,
  };
};
