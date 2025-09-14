import { invokeTauriIpc } from "@/lib/backend/tauri";
import { StreamEntriesEvent } from "@repo/moss-project";
import { Channel } from "@tauri-apps/api/core";

export const startStreamingProjectEntries = async (projectId: string, path?: string): Promise<StreamEntriesEvent[]> => {
  const entries: StreamEntriesEvent[] = [];
  const onProjectEntryEvent = new Channel<StreamEntriesEvent>();

  onProjectEntryEvent.onmessage = (projectEntry) => {
    entries.push(projectEntry);
  };

  const result = await invokeTauriIpc("stream_project_entries", {
    collectionId: projectId,
    channel: onProjectEntryEvent,
    input: path ? { "RELOAD_PATH": path } : "LOAD_ROOT",
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return entries;
};
