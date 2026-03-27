import {
  BatchCreateResourceKind,
  BodyInfo,
  HeaderInfo,
  PathParamInfo,
  QueryParamInfo,
  ResourceClass,
  ResourceProtocol,
} from "@repo/moss-project";

interface CreateResourceKindProps {
  name: string;
  path: string;
  class: ResourceClass;
  isAddingFolder: boolean;
  order: number;

  protocol?: ResourceProtocol;
  headers?: HeaderInfo[];
  queryParams?: QueryParamInfo[];
  pathParams?: PathParamInfo[];
  body?: BodyInfo;
}

export const createResourceKind = ({
  name,
  path,
  isAddingFolder,
  class: resourceClass,
  order,
  protocol,
  headers,
  queryParams,
  pathParams,
  body,
}: CreateResourceKindProps): BatchCreateResourceKind => {
  if (isAddingFolder) {
    return { DIR: { name, path, class: resourceClass, order } };
  }

  return {
    ITEM: {
      name,
      path,
      class: resourceClass,
      order,
      protocol,
      headers:
        headers?.map(({ name, value, order }) => ({
          name,
          value,
          order: order ?? 0,
          options: { disabled: false, propagate: false },
        })) ?? [],
      queryParams:
        queryParams?.map(({ name, value, order }) => ({
          name,
          value,
          order: order ?? 0,
          options: { disabled: false, propagate: false },
        })) ?? [],
      pathParams:
        pathParams?.map(({ name, value, order }) => ({
          name,
          value,
          order: order ?? 0,
          options: { disabled: false, propagate: false },
        })) ?? [],
    },
  };
};
