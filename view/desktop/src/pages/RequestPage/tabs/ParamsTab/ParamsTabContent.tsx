import React from "react";
import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { EntryKind } from "@repo/moss-collection";
import { IDockviewPanelProps } from "@repo/moss-tabs";
import { DataTable, ParameterData } from "@/components/Table";
import { paramColumns } from "./columns";
import { getParameterSuggestions, detectValueType } from "../../utils/urlParser";
import { useRequestPageStore } from "@/store/requestPage";

interface ParamsTabContentProps
  extends IDockviewPanelProps<{
    node?: TreeCollectionNode;
    treeId: string;
    iconType: EntryKind;
    someRandomString: string;
  }> {}

export const ParamsTabContent = (_props: ParamsTabContentProps) => {
  const { requestData, updatePathParams, updateQueryParams, reconstructUrlFromParams } = useRequestPageStore();

  const debouncedQueryUpdate = React.useRef<NodeJS.Timeout>();
  const debouncedPathUpdate = React.useRef<NodeJS.Timeout>();

  const convertUrlParamsToTableData = (
    params: Array<{ key: string; value: string }>,
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
        properties: { disabled: false },
      };
    });
  };

  const queryParams = convertUrlParamsToTableData(requestData.url.query_params, "query");
  const pathParams = convertUrlParamsToTableData(requestData.url.path_params, "path");

  const handleQueryParamsUpdate = React.useCallback(
    (updatedData: ParameterData[]) => {
      if (debouncedQueryUpdate.current) {
        clearTimeout(debouncedQueryUpdate.current);
      }

      // Debounce to prevent focus loss during typing
      debouncedQueryUpdate.current = setTimeout(() => {
        const updatedParams = updatedData
          .filter((param) => param.key.trim() !== "" || param.value.trim() !== "")
          .map((param) => ({
            key: param.key,
            value: param.value,
          }));

        const currentParams = requestData.url.query_params;
        const paramsChanged = JSON.stringify(updatedParams) !== JSON.stringify(currentParams);

        if (paramsChanged) {
          updateQueryParams(updatedParams);
          reconstructUrlFromParams();
        }
      }, 300);
    },
    [updateQueryParams, reconstructUrlFromParams, requestData.url.query_params]
  );

  const handlePathParamsUpdate = React.useCallback(
    (updatedData: ParameterData[]) => {
      if (debouncedPathUpdate.current) {
        clearTimeout(debouncedPathUpdate.current);
      }

      debouncedPathUpdate.current = setTimeout(() => {
        const updatedParams = updatedData
          .filter((param) => param.key.trim() !== "" || param.value.trim() !== "")
          .map((param) => ({
            key: param.key,
            value: param.value,
          }));

        const currentParams = requestData.url.path_params;
        const paramsChanged = JSON.stringify(updatedParams) !== JSON.stringify(currentParams);

        if (paramsChanged) {
          updatePathParams(updatedParams);
          reconstructUrlFromParams();
        }
      }, 300);
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
      {/* Query Params */}
      <div className="mb-6">
        <h3 className="mb-3 text-sm font-medium">Query Params</h3>
        <DataTable columns={paramColumns} data={queryParams} onDataChange={handleQueryParamsUpdate} />
      </div>

      {/* Path Params */}
      <div>
        <h3 className="mb-3 text-sm font-medium">Path Params</h3>
        <DataTable columns={paramColumns} data={pathParams} onDataChange={handlePathParamsUpdate} />
      </div>
    </div>
  );
};
