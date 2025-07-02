import { invokeTauriIpc } from "@/lib/backend/tauri";
import { getClassAndProtocolFromEntyInput } from "@/utils/getClassAndProtocolFromEntyInput";
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

  const { entryClass, protocol } = getClassAndProtocolFromEntyInput(input);

  //FIXME: This is a temporary solution until we have a proper configuration model
  if ("dir" in input) {
    const rawpath = await join(input.dir.path, input.dir.name);

    const newEntry: EntryInfo = {
      id: result.data.id,
      name: input.dir.name,
      order: input.dir.order ?? 0, // TODO:order should always be set
      path: {
        raw: rawpath,
        segments: rawpath.split(sep()),
      },
      class: entryClass,
      kind: "Dir" as const,
      expanded: false,
    };

    return newEntry;
  } else if ("item" in input) {
    const rawpath = await join(input.item.path, input.item.name);

    const newEntry: EntryInfo = {
      id: result.data.id,
      name: input.item.name,
      order: input.item.order ?? undefined,
      path: {
        raw: rawpath,
        segments: rawpath.split(sep()),
      },
      class: entryClass,
      kind: "Item" as const,
      protocol,
      expanded: false,
    };

    return newEntry;
  }

  return null;
};

export const useCreateCollectionEntry = () => {
  const queryClient = useQueryClient();

  return useMutation<EntryInfo | null, Error, UseCreateCollectionEntryInputProps>({
    mutationFn: createCollectionEntry,
    onSuccess: (data, variables) => {
      queryClient.setQueryData(
        [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, variables.collectionId],
        (old: EntryInfo[]) => {
          return [...old, data];
        }
      );
    },
  });
};
