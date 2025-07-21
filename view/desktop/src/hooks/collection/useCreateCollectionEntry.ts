import { invokeTauriIpc } from "@/lib/backend/tauri";
import { getClassAndProtocolFromEntryInput } from "@/utils/getClassAndProtocolFromEntyInput";
import { CreateEntryInput, CreateEntryOutput, EntryInfo } from "@repo/moss-collection";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { join, sep } from "@tauri-apps/api/path";

import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY } from "./useStreamedCollectionEntries";

export interface UseCreateCollectionEntryInputProps {
  collectionId: string;
  input: CreateEntryInput;
}

const createCollectionEntry = async ({ collectionId, input }: UseCreateCollectionEntryInputProps) => {
  const result = await invokeTauriIpc<CreateEntryOutput>("create_collection_entry", {
    collectionId,
    input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCreateCollectionEntry = () => {
  const queryClient = useQueryClient();

  return useMutation<CreateEntryOutput, Error, UseCreateCollectionEntryInputProps>({
    mutationFn: createCollectionEntry,
    onSuccess: async (data, variables) => {
      const newEntry = await createNewEntry(data.id, variables.input);

      queryClient.setQueryData(
        [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, variables.collectionId],
        (old: EntryInfo[]) => {
          return [...old, newEntry];
        }
      );
    },
  });
};

const createNewEntry = async (id: string, entry: CreateEntryInput): Promise<EntryInfo> => {
  //FIXME: This is a temporary solution until we have a proper configuration model
  const { entryClass, protocol } = getClassAndProtocolFromEntryInput(entry);
  if ("DIR" in entry) {
    const rawpath = await join(entry.DIR.path, entry.DIR.name);

    return {
      id,
      name: entry.DIR.name,
      order: entry.DIR.order,
      path: {
        raw: rawpath,
        segments: rawpath.split(sep()),
      },
      class: entryClass,
      kind: "Dir",
      expanded: false,
    };
  } else {
    const rawpath = await join(entry.ITEM.path, entry.ITEM.name);

    return {
      id,
      name: entry.ITEM.name,
      order: entry.ITEM.order,
      path: {
        raw: rawpath,
        segments: rawpath.split(sep()),
      },
      class: entryClass,
      kind: "Item" as const,
      protocol,
      expanded: false,
    };
  }
};
