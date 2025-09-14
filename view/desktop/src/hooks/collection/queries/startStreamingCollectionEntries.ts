import { invokeTauriIpc } from "@/lib/backend/tauri";
import { StreamEntriesEvent } from "@repo/moss-project";
import { Channel } from "@tauri-apps/api/core";

export const startStreamingCollectionEntries = async (
  collectionId: string,
  path?: string
): Promise<StreamEntriesEvent[]> => {
  const entries: StreamEntriesEvent[] = [];
  const onCollectionEntryEvent = new Channel<StreamEntriesEvent>();

  onCollectionEntryEvent.onmessage = (collectionEntry) => {
    entries.push(collectionEntry);
  };

  const result = await invokeTauriIpc("stream_project_entries", {
    collectionId,
    channel: onCollectionEntryEvent,
    input: path ? { "RELOAD_PATH": path } : "LOAD_ROOT",
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return entries;
};
