import { CreateEntryInput, StreamEntriesEvent } from "@repo/moss-collection";
import { join, sep } from "@tauri-apps/api/path";

//FIXME: This is a temporary solution until we have a proper configuration model
export const createCollectionEntryForCache = async (
  id: string,
  entry: CreateEntryInput
): Promise<StreamEntriesEvent> => {
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
      class: entry.DIR.class,
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
      class: entry.ITEM.class,
      kind: "Item" as const,
      protocol: entry.ITEM.protocol,
      expanded: false,
    };
  }
};
