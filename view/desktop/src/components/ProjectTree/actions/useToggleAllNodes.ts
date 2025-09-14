import { useContext } from "react";

import { USE_STREAM_PROJECT_ENTRIES_QUERY_KEY, useStreamProjectEntries } from "@/hooks";
import { useBatchUpdateProjectEntry } from "@/hooks/project/useBatchUpdateProjectEntry";
import { BatchUpdateEntryKind, StreamEntriesEvent } from "@repo/moss-project";
import { useQueryClient } from "@tanstack/react-query";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeRootNode } from "../types";

export const useToggleAllNodes = (node: ProjectTreeRootNode) => {
  const { id } = useContext(ProjectTreeContext);

  const queryClient = useQueryClient();

  const { data: streamedEntries } = useStreamProjectEntries(id);
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateProjectEntry();

  const expandAllNodes = async () => {
    if (!streamedEntries) return;

    const entriesToUpdate = streamedEntries.filter((entry) => !entry.expanded && entry.kind === "Dir");

    const inputEntries = entriesToUpdate.map((entry): BatchUpdateEntryKind => {
      if (entry.kind === "Dir") {
        return {
          DIR: {
            id: entry.id,
            expanded: true,
          },
        };
      } else {
        return {
          ITEM: {
            id: entry.id,
            expanded: true,
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
        };
      }
    });

    await batchUpdateCollectionEntry({
      projectId: node.id,
      entries: {
        entries: inputEntries,
      },
    });

    queryClient.setQueryData([USE_STREAM_PROJECT_ENTRIES_QUERY_KEY, id], (oldEntries: StreamEntriesEvent[]) => {
      return oldEntries.map((entry) => {
        if (entry.kind === "Dir" && !entry.expanded) {
          return { ...entry, expanded: true };
        }
        return entry;
      });
    });
  };

  const collapseAllNodes = async () => {
    if (!streamedEntries) return;

    const entriesToUpdate = streamedEntries.filter((entry) => entry.expanded && entry.kind === "Dir");

    const inputEntries = entriesToUpdate.map((entry): BatchUpdateEntryKind => {
      if (entry.kind === "Dir") {
        return {
          DIR: {
            id: entry.id,
            expanded: false,
          },
        };
      } else {
        return {
          ITEM: {
            id: entry.id,
            expanded: false,
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
        };
      }
    });

    await batchUpdateCollectionEntry({
      projectId: node.id,
      entries: {
        entries: inputEntries,
      },
    });

    queryClient.setQueryData([USE_STREAM_PROJECT_ENTRIES_QUERY_KEY, id], (oldEntries: StreamEntriesEvent[]) => {
      return oldEntries.map((entry) => {
        if (entry.kind === "Dir" && entry.expanded) {
          return { ...entry, expanded: false };
        }
        return entry;
      });
    });
  };

  return { expandAllNodes, collapseAllNodes };
};
