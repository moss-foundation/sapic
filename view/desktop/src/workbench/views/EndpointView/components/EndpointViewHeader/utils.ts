import { ResourceDetails } from "@/db/resource/types";
import {
  DescribeResourceOutput,
  PathParamInfo,
  QueryParamInfo,
  UpdateItemResourceParams,
  UpdatePathParamParams,
  UpdateQueryParamParams,
} from "@repo/moss-project";

export const buildPathParamUpdateObject = (initial: PathParamInfo, updated: PathParamInfo): UpdatePathParamParams => {
  const updateObj: UpdatePathParamParams = { id: initial.id };

  if (initial.name !== updated.name) updateObj.name = updated.name;

  if (initial.value !== updated.value)
    updateObj.value = {
      "UPDATE": updated.value,
    };

  if (initial.description !== updated.description && updated.description)
    updateObj.description = {
      "UPDATE": updated.description,
    };

  if (initial.order !== updated.order) updateObj.order = updated.order;

  const optionsChanged = initial.disabled !== updated.disabled || initial.propagate !== updated.propagate;

  if (optionsChanged) {
    updateObj.options = {
      disabled: updated.disabled,
      propagate: updated.propagate,
    };
  }

  return updateObj;
};

export const buildQueryParamUpdateObject = (
  initial: QueryParamInfo,
  updated: QueryParamInfo
): UpdateQueryParamParams => {
  const updateObj: UpdateQueryParamParams = { id: initial.id };

  if (initial.name !== updated.name) updateObj.name = updated.name;

  if (initial.value !== updated.value)
    updateObj.value = {
      "UPDATE": updated.value,
    };

  if (initial.description !== updated.description && updated.description)
    updateObj.description = {
      "UPDATE": updated.description,
    };

  if (initial.order !== updated.order) updateObj.order = updated.order;

  const optionsChanged = initial.disabled !== updated.disabled || initial.propagate !== updated.propagate;

  if (optionsChanged) {
    updateObj.options = {
      disabled: updated.disabled,
      propagate: updated.propagate,
    };
  }

  return updateObj;
};

export const buildDescriptionParamsToAdd = (
  localResourceDetails: ResourceDetails,
  backendResourceDetails: DescribeResourceOutput
) => {
  const updateObj: Partial<UpdateItemResourceParams> = {};
  if (localResourceDetails.name !== backendResourceDetails.name) updateObj.name = localResourceDetails.name;
  if (localResourceDetails.protocol !== backendResourceDetails.protocol)
    updateObj.protocol = localResourceDetails.protocol;
  //TODO this is not supported yet by the backend
  // if(localResourceDetails.body) updateObj.body = localResourceDetails.body;
  // if(localResourceDetails.order) updateObj.order = localResourceDetails.order;
  // if(localResourceDetails.path) updateObj.path = localResourceDetails.path;

  if (Object.keys(updateObj).length === 0) {
    return null;
  }

  return updateObj;
};
