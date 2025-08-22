import { CreateEntryInput, StreamEntriesEvent } from "@repo/moss-collection";
import { join, sep } from "@tauri-apps/api/path";

//FIXME: This is a temporary solution until we have a proper configuration model
export const createCollectionEntryForCache = async (
  id: string,
  entry: CreateEntryInput
): Promise<StreamEntriesEvent> => {
  const { entryClass } = getClassFromEntryInput(entry);

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
      protocol: entry.ITEM.protocol,
      expanded: false,
    };
  }
};

const getClassFromEntryInput = (input: CreateEntryInput) => {
  const pathSource = "DIR" in input ? input.DIR : input.ITEM;
  const firstPath = pathSource.path.split(sep())[0];

  const pathToClassMap: Record<string, "Request" | "Endpoint" | "Component" | "Schema"> = {
    requests: "Request",
    endpoints: "Endpoint",
    components: "Component",
    schemas: "Schema",
  };

  const entryClass = pathToClassMap[firstPath];
  if (!entryClass) {
    throw new Error("Invalid path in CreateEntryInput");
  }

  return { entryClass };
};
