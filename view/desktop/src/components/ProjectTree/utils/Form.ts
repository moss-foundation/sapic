import {
  BatchCreateEntryKind,
  CreateEntryInput,
  EntryClass,
  EntryProtocol,
  StreamEntriesEvent,
} from "@repo/moss-project";

interface CreateEntryKindProps {
  name: string;
  path: string;
  class: EntryClass;
  isAddingFolder: boolean;
  order: number;
  protocol?: EntryProtocol;
}

export const createEntryKind = ({
  name,
  path,
  isAddingFolder,
  class: entryClass,
  order,
  protocol,
}: CreateEntryKindProps): BatchCreateEntryKind => {
  if (isAddingFolder) {
    return {
      DIR: {
        name,
        path,
        class: entryClass,
        order,
        headers: [],
      },
    };
  }

  return {
    ITEM: {
      name,
      path,
      class: entryClass,
      order,
      headers: [],
      queryParams: [],
      pathParams: [],
      protocol,
    },
  };
};

export const convertEntryInfoToCreateInput = (
  entry: StreamEntriesEvent,
  newProjectPath: string = ""
): CreateEntryInput => {
  if (entry.kind === "Dir") {
    return {
      DIR: {
        name: entry.name,
        path: newProjectPath,
        class: entry.class,
        order: entry.order ?? 0,
        headers: [],
      },
    };
  } else {
    return {
      ITEM: {
        name: entry.name,
        path: newProjectPath,
        class: entry.class,
        order: entry.order ?? 0,
        protocol: entry.protocol ?? "Get",
        headers: [],
        queryParams: [],
        pathParams: [],
      },
    };
  }
};
