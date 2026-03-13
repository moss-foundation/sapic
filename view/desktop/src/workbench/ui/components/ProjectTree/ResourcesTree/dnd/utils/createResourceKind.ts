import { BatchCreateResourceKind, ResourceClass, ResourceProtocol } from "@repo/moss-project";

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
