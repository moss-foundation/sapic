import { ParsedValue } from "@/workbench/views/EndpointView/utils";
import { QueryParamInfo } from "@repo/moss-project";

export const extractParsedValueString = (parsedValues: Array<ParsedValue>): string => {
  return parsedValues
    .map((value) => {
      if ("variable" in value) return value.variable;
      if ("pathVariable" in value) return value.pathVariable;
      return value.string;
    })
    .join("");
};

export const createPathParam = (
  pathParamName: string,
  index: number,
  existingParam?: QueryParamInfo
): QueryParamInfo => {
  if (existingParam) return existingParam;

  return {
    id: `path-param-${pathParamName}`,
    name: pathParamName,
    value: "",
    disabled: false,
    propagate: false,
    order: index,
    description: "",
  };
};

export const createQueryParam = (
  queryParamName: string,
  queryParamValue: string,
  index: number,
  existingParam?: QueryParamInfo
): QueryParamInfo => {
  if (existingParam) {
    return {
      ...existingParam,
      value: queryParamValue,
    };
  }

  return {
    id: Math.random().toString(36).substring(2, 15),
    name: queryParamName,
    value: queryParamValue,
    disabled: false,
    propagate: false,
    order: index + 1,
    description: "",
  };
};
