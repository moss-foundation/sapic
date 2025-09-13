import { BatchCreateEntryKind, EntryClass, EntryProtocol } from "@repo/moss-collection";

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
