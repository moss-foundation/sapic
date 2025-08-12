import { useContext } from "react";

import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, useStreamedCollectionEntries } from "@/hooks";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { BatchUpdateEntryKind, EntryInfo } from "@repo/moss-collection";
import { useQueryClient } from "@tanstack/react-query";

import { TreeContext } from "../Tree";
import { TreeCollectionRootNode } from "../types";

export const useToggleAllNodes = (node: TreeCollectionRootNode) => {
  const { id } = useContext(TreeContext);

  const queryClient = useQueryClient();

  const { data: streamedEntries } = useStreamedCollectionEntries(id);
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateCollectionEntry();

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
      collectionId: node.id,
      entries: {
        entries: inputEntries,
      },
    });

    queryClient.setQueryData([USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, id], (oldEntries: EntryInfo[]) => {
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
      collectionId: node.id,
      entries: {
        entries: inputEntries,
      },
    });

    queryClient.setQueryData([USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, id], (oldEntries: EntryInfo[]) => {
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
