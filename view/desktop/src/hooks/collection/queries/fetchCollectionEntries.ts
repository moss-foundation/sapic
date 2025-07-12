import { invokeTauriIpc } from "@/lib/backend/tauri";
import { EntryInfo } from "@repo/moss-collection";
import { Channel } from "@tauri-apps/api/core";

export const fetchCollectionEntries = async (collectionId: string): Promise<EntryInfo[]> => {
  const entries: EntryInfo[] = [];
  const onCollectionEntryEvent = new Channel<EntryInfo>();

  onCollectionEntryEvent.onmessage = (collectionEntry) => {
    entries.push(collectionEntry);
  };

  const result = await invokeTauriIpc("stream_collection_entries", {
    collectionId,
    channel: onCollectionEntryEvent,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return entries;
};
