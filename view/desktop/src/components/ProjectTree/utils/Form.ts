import {
  BatchCreateResourceKind,
  CreateResourceInput,
  ResourceClass,
  ResourceProtocol,
  StreamResourcesEvent,
} from "@repo/moss-project";

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
  resource: StreamResourcesEvent,
  newProjectPath: string = ""
): CreateResourceInput => {
  if (resource.kind === "Dir") {
    return {
      DIR: {
        name: resource.name,
        path: newProjectPath,
        class: resource.class,
        order: resource.order ?? 0,
      },
    };
  } else {
    return {
      ITEM: {
        name: resource.name,
        path: newProjectPath,
        class: resource.class,
        order: resource.order ?? 0,
        protocol: resource.protocol ?? "Get",
        headers: [],
        queryParams: [],
        pathParams: [],
      },
    };
  }
};
