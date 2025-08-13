import { BatchCreateEntryKind, CreateEntryInput, EntryInfo, EntryProtocol } from "@repo/moss-collection";

interface CreateEntryKindProps {
  name: string;
  path: string;
  isAddingFolder: boolean;
  order: number;
  protocol?: EntryProtocol;
}

export const createEntryKind = ({
  name,
  path,
  isAddingFolder,
  order,
  protocol,
}: CreateEntryKindProps): BatchCreateEntryKind => {
  if (isAddingFolder) {
    return {
      DIR: {
        name,
        path,
        order,
        headers: [],
      },
    };
  }

  return {
    ITEM: {
      name,
      path,
      order,
      headers: [],
      queryParams: [],
      pathParams: [],
      protocol,
    },
  };
};

export const convertEntryInfoToCreateInput = (
  entry: EntryInfo,
  newCollectionPath: string = "requests"
): CreateEntryInput => {
  if (entry.kind === "Dir") {
    return {
      DIR: {
        name: entry.name,
        path: newCollectionPath,
        order: entry.order ?? 0,
        headers: [],
      },
    };
  } else {
    return {
      ITEM: {
        name: entry.name,
        path: newCollectionPath,
        order: entry.order ?? 0,
        headers: [],
        queryParams: [],
        pathParams: [],
      },
    };
  }
};
