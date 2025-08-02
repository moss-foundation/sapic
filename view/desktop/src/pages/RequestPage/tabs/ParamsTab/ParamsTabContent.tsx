import React from "react";
import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { EntryKind } from "@repo/moss-collection";
import { IDockviewPanelProps } from "@repo/moss-tabs";
import { DataTable, ParameterData } from "@/components/Table";
import { ActionButton } from "@/components";
import { paramColumns } from "./columns";
import { getParameterSuggestions, detectValueType } from "../../utils/urlParser";
import { useRequestPageStore } from "@/store/requestPage";

interface ParamsTabContentProps
  extends IDockviewPanelProps<{
    node?: TreeCollectionNode;
    collectionId: string;
    iconType: EntryKind;
    someRandomString: string;
  }> {}

export const ParamsTabContent = (_props: ParamsTabContentProps) => {
  const { requestData, updatePathParams, updateQueryParams, reconstructUrlFromParams } = useRequestPageStore();

  const debouncedQueryUpdate = React.useRef<NodeJS.Timeout>();
  const debouncedPathUpdate = React.useRef<NodeJS.Timeout>();

  const convertUrlParamsToTableData = (
    params: Array<{ key: string; value: string; disabled?: boolean }>,
    type: "path" | "query"
  ): ParameterData[] => {
    return params.map((param, index) => {
      const paramKey = param.key || "";
      const paramValue = param.value || "";

      let detectedType = "string";
      if (paramValue) {
        detectedType = detectValueType(paramValue);
      } else if (paramKey) {
        const suggestions = getParameterSuggestions(paramKey);
        detectedType = suggestions.type;
      }

      const suggestions = getParameterSuggestions(paramKey || "param");

      return {
        order: index + 1,
        id: `${type}-${index}-${Date.now()}`,
        key: paramKey,
        value: paramValue,
        type: detectedType,
        description: paramKey ? suggestions.description : `${type === "path" ? "Path" : "Query"} parameter`,
        global_value: "",
        local_value: 0,
        properties: { disabled: param.disabled || false },
      };
    });
  };

  const queryParams = React.useMemo(
    () => convertUrlParamsToTableData(requestData.url.query_params, "query"),
    [requestData.url.query_params]
  );
  const pathParams = React.useMemo(
    () => convertUrlParamsToTableData(requestData.url.path_params, "path"),
    [requestData.url.path_params]
  );

  const handleQueryParamsUpdate = React.useCallback(
    (updatedData: ParameterData[]) => {
      if (debouncedQueryUpdate.current) {
        clearTimeout(debouncedQueryUpdate.current);
      }

      const previousData = queryParams;
      const isCheckboxChange =
        updatedData.length === previousData.length &&
        updatedData.some(
          (param, index) =>
            previousData[index] &&
            param.key === previousData[index].key &&
            param.value === previousData[index].value &&
            param.properties.disabled !== previousData[index].properties.disabled
        );

      const debounceDelay = isCheckboxChange ? 30 : 100;

      debouncedQueryUpdate.current = setTimeout(() => {
        const updatedParams = updatedData
          .filter((param) => param.key.trim() !== "" || param.value.trim() !== "")
          .map((param) => ({
            key: param.key,
            value: param.value,
            disabled: param.properties.disabled,
          }));

        const currentParams = requestData.url.query_params;
        const paramsChanged = JSON.stringify(updatedParams) !== JSON.stringify(currentParams);

        if (paramsChanged) {
          updateQueryParams(updatedParams);
          reconstructUrlFromParams();
        }
      }, debounceDelay);
    },
    [updateQueryParams, reconstructUrlFromParams, requestData.url.query_params]
  );

  const handlePathParamsUpdate = React.useCallback(
    (updatedData: ParameterData[]) => {
      if (debouncedPathUpdate.current) {
        clearTimeout(debouncedPathUpdate.current);
      }

      const previousData = pathParams;
      const isCheckboxChange =
        updatedData.length === previousData.length &&
        updatedData.some(
          (param, index) =>
            previousData[index] &&
            param.key === previousData[index].key &&
            param.value === previousData[index].value &&
            param.properties.disabled !== previousData[index].properties.disabled
        );

      const debounceDelay = isCheckboxChange ? 30 : 100;

      debouncedPathUpdate.current = setTimeout(() => {
        const updatedParams = updatedData
          .filter((param) => param.key.trim() !== "")
          .map((param) => ({
            key: param.key,
            value: param.value.trim() !== "" ? param.value : "",
            disabled: param.properties.disabled,
          }));

        const currentParams = requestData.url.path_params;
        const paramsChanged = JSON.stringify(updatedParams) !== JSON.stringify(currentParams);

        if (paramsChanged) {
          updatePathParams(updatedParams);
          reconstructUrlFromParams();
        }
      }, debounceDelay);
    },
    [updatePathParams, reconstructUrlFromParams, requestData.url.path_params]
  );

  React.useEffect(() => {
    return () => {
      if (debouncedQueryUpdate.current) {
        clearTimeout(debouncedQueryUpdate.current);
      }
      if (debouncedPathUpdate.current) {
        clearTimeout(debouncedPathUpdate.current);
      }
    };
  }, []);

  return (
    <div className="mt-4">
      <div className="mb-6">
        <div className="mb-3 flex items-center justify-between">
          <h3 className="text-sm font-medium">Query Params</h3>
          <ActionButton icon="MoreHorizontal" />
        </div>
        <DataTable columns={paramColumns} data={queryParams} onDataChange={handleQueryParamsUpdate} />
      </div>

      <div>
        <div className="mb-3 flex items-center justify-between">
          <h3 className="text-sm font-medium">Path Params</h3>
          <ActionButton icon="MoreHorizontal" />
        </div>
        <DataTable columns={paramColumns} data={pathParams} onDataChange={handlePathParamsUpdate} />
      </div>
    </div>
  );
};
