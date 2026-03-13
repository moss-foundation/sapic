import { ListProjectResourceItem } from "@repo/ipc";
import { CreateResourceInput } from "@repo/moss-project";

export const convertResourceToCreateInput = (
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
