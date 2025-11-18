import { CreateResourceInput, StreamResourcesEvent } from "@repo/moss-project";
import { join, sep } from "@tauri-apps/api/path";

//FIXME: This is a temporary solution until we have a proper configuration model
export const createProjectResourceForCache = async (
  id: string,
  resource: CreateResourceInput
): Promise<StreamResourcesEvent> => {
  if ("DIR" in resource) {
    const rawpath = await join(resource.DIR.path, resource.DIR.name);

    return {
      id,
      name: resource.DIR.name,
      order: resource.DIR.order,
      path: {
        raw: rawpath,
        segments: rawpath.split(sep()),
      },
      class: resource.DIR.class,
      kind: "Dir",
      expanded: false,
    };
  } else {
    const rawpath = await join(resource.ITEM.path, resource.ITEM.name);

    return {
      id,
      name: resource.ITEM.name,
      order: resource.ITEM.order,
      path: {
        raw: rawpath,
        segments: rawpath.split(sep()),
      },
      class: resource.ITEM.class,
      kind: "Item" as const,
      protocol: resource.ITEM.protocol,
      expanded: false,
    };
  }
};
