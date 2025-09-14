import { useState } from "react";

import { useFetchEntriesForPath } from "@/hooks/project/derivedHooks/useFetchEntriesForPath";
import { useUpdateProjectEntry } from "@/hooks/project/useUpdateProjectEntry";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamEntriesEvent } from "@repo/moss-project";
import { join } from "@tauri-apps/api/path";

export const useRenameEntryForm = (entry: StreamEntriesEvent, projectId: string) => {
  const { fetchEntriesForPath } = useFetchEntriesForPath();
  const { mutateAsync: updateProjectEntry } = useUpdateProjectEntry();
  const { api } = useTabbedPaneStore();

  const [isRenamingEntry, setIsRenamingEntry] = useState(false);

  const handleRenamingEntrySubmit = async (newName: string) => {
    try {
      const trimmedNewName = newName.trim();

      if (trimmedNewName === entry.name) {
        return;
      }

      if (entry.kind === "Dir") {
        await updateProjectEntry({
          projectId,
          updatedEntry: {
            DIR: {
              id: entry.id,
              name: trimmedNewName,
            },
          },
        });

        const newPath = await join(...entry.path.segments.slice(0, entry.path.segments.length - 1), trimmedNewName);
        await fetchEntriesForPath(projectId, newPath);
      } else {
        await updateProjectEntry({
          projectId,
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
