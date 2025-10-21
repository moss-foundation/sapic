import {
  BatchCreateResourceKind,
  CreateResourceInput,
  ResourceClass,
  ResourceProtocol,
  StreamResourcesEvent,
} from "@repo/moss-project";

interface CreateEntryKindProps {
  name: string;
  path: string;
  class: ResourceClass;
  isAddingFolder: boolean;
  order: number;
  protocol?: ResourceProtocol;
}

export const createEntryKind = ({
  name,
  path,
  isAddingFolder,
  class: entryClass,
  order,
  protocol,
}: CreateEntryKindProps): BatchCreateResourceKind => {
  if (isAddingFolder) {
    return {
      DIR: {
        name,
        path,
        class: entryClass,
        order,
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
  entry: StreamResourcesEvent,
  newProjectPath: string = ""
): CreateResourceInput => {
  if (entry.kind === "Dir") {
    return {
      DIR: {
        name: entry.name,
        path: newProjectPath,
        class: entry.class,
        order: entry.order ?? 0,
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
