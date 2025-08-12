import {
  BatchCreateEntryKind,
  CreateEntryInput,
  DirConfigurationModel,
  EntryInfo,
  ItemConfigurationModel,
} from "@repo/moss-collection";

export const createEntryKind = (
  name: string,
  path: string,
  isAddingFolder: boolean,
  entryClass: EntryInfo["class"],
  order: number
): BatchCreateEntryKind => {
  if (isAddingFolder) {
    return {
      DIR: {
        name,
        path,
        order,
        configuration: createDirConfiguration(entryClass),
      },
    };
  }

  return {
    ITEM: {
      name,
      path,
      order,
      configuration: createItemConfiguration(entryClass),
    },
  };
};

//FIXME: This is a temporary solution until we have a proper configuration model
export const createDirConfiguration = (entryClass: EntryInfo["class"]): DirConfigurationModel => {
  switch (entryClass) {
    case "Request":
      return { request: { http: {} } };
    case "Endpoint":
      return { request: { http: {} } };
    case "Component":
      return { component: {} };
    case "Schema":
      return { schema: {} };
    default:
      return { request: { http: {} } };
  }
};

//FIXME: This is a temporary solution until we have a proper configuration model
export const createItemConfiguration = (entryClass: EntryInfo["class"]): ItemConfigurationModel => {
  switch (entryClass) {
    case "Request":
      return {
        request: {
          http: {
            requestParts: {
              method: "GET",
            },
          },
        },
      };
    case "Endpoint":
      return {
        endpoint: {
          Http: {
            requestParts: {
              method: "GET",
            },
          },
        },
      };
    case "Component":
      return { component: {} };
    case "Schema":
      return { schema: {} };
    default:
      return {
        request: {
          http: {
            requestParts: {
              method: "GET",
            },
          },
        },
      };
  }
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
        configuration: {
          request: {
            http: {},
          },
        },
      },
    };
  } else {
    return {
      ITEM: {
        name: entry.name,
        path: newCollectionPath,
        order: entry.order ?? 0,
        configuration: {
          request: {
            http: {
              requestParts: {
                method: "GET",
              },
            },
          },
        },
      },
    };
  }
};
