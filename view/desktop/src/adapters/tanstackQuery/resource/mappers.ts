import { ResourceDetails } from "@/db/resourceDetails/types";
import { BodyInfo, UpdateBodyParams, UpdateResourceInput } from "@repo/moss-project";

export const mapUpdateResourceInputToResourceDetails = (
  updateResourceInput: UpdateResourceInput,
  existingResourceDetails?: Partial<ResourceDetails>
): ResourceDetails => {
  if ("ITEM" in updateResourceInput) {
    const item = updateResourceInput.ITEM;
    return {
      id: item.id,
      name: item.name ?? existingResourceDetails?.name ?? "",
      class: existingResourceDetails?.class ?? "endpoint",
      kind: existingResourceDetails?.kind ?? "Item",
      protocol: item.protocol ?? existingResourceDetails?.protocol,
      url: existingResourceDetails?.url,
      headers: existingResourceDetails?.headers ?? [],
      pathParams: existingResourceDetails?.pathParams ?? [],
      queryParams: existingResourceDetails?.queryParams ?? [],
      body: item.body !== undefined ? mapUpdateBodyParamsToBodyInfo(item.body) : existingResourceDetails?.body,
      metadata: existingResourceDetails?.metadata ?? { isDirty: false },
    };
  } else {
    const dir = updateResourceInput.DIR;
    return {
      id: dir.id,
      name: dir.name ?? existingResourceDetails?.name ?? "",
      class: existingResourceDetails?.class ?? "endpoint",
      kind: existingResourceDetails?.kind ?? "Dir",
      protocol: existingResourceDetails?.protocol,
      url: existingResourceDetails?.url,
      headers: existingResourceDetails?.headers ?? [],
      pathParams: existingResourceDetails?.pathParams ?? [],
      queryParams: existingResourceDetails?.queryParams ?? [],
      body: existingResourceDetails?.body,
      metadata: existingResourceDetails?.metadata ?? { isDirty: false },
    };
  }
};

const mapUpdateBodyParamsToBodyInfo = (updateBodyParams: UpdateBodyParams): BodyInfo | undefined => {
  if (updateBodyParams === "remove") {
    return undefined;
  }
  if ("text" in updateBodyParams) {
    return { text: updateBodyParams.text };
  }
  if ("json" in updateBodyParams) {
    return { json: updateBodyParams.json };
  }
  if ("xml" in updateBodyParams) {
    return { xml: updateBodyParams.xml };
  }
  if ("binary" in updateBodyParams) {
    return { binary: updateBodyParams.binary };
  }
  if ("urlencoded" in updateBodyParams) {
    // UpdateBodyParams.urlencoded contains params_to_add, params_to_update, params_to_remove
    // This would need to be merged with existing params, so we return undefined here
    // and let the caller handle the merge logic
    return undefined;
  }
  if ("formData" in updateBodyParams) {
    // Similar to urlencoded, this needs merge logic
    return undefined;
  }
  return undefined;
};
