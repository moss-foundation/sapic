import { ListProjectResourceItem } from "@repo/ipc";
import { BatchCreateResourceKind, CreateResourceInput, ResourceClass, ResourceProtocol } from "@repo/moss-project";

interface CreateResourceKindProps {
  name: string;
  path: string;
  class: ResourceClass;
  isAddingFolder: boolean;
  order: number;
  protocol?: ResourceProtocol;
}

export const createResourceKind = ({
  name,
  path,
  isAddingFolder,
  class: resourceClass,
  order,
  protocol,
}: CreateResourceKindProps): BatchCreateResourceKind => {
  if (isAddingFolder) {
    return {
      DIR: {
        name,
        path,
        class: resourceClass,
        order,
      },
    };
  }

  return {
    ITEM: {
      name,
      path,
      class: resourceClass,
      order,
      headers: [],
      queryParams: [],
      pathParams: [],
      protocol,
    },
  };
};

export const convertResourceInfoToCreateInput = (
  resource: ListProjectResourceItem,
  newProjectPath: string = ""
): CreateResourceInput => {
  if (resource.kind === "Dir") {
    return {
      DIR: {
        name: resource.name,
        path: newProjectPath,
        class: resource.class,
        order: -1,
      },
    };
  } else {
    return {
      ITEM: {
        name: resource.name,
        path: newProjectPath,
        class: resource.class,
        protocol: resource.protocol ?? "Get",
        order: -1,
        headers: [],
        queryParams: [],
        pathParams: [],
      },
    };
  }
};
