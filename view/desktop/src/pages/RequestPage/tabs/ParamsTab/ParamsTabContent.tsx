import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { EntryKind } from "@repo/moss-collection";
import { IDockviewPanelProps } from "@repo/moss-tabs";
import { DataTable, ParameterData } from "@/components/Table";
import { paramColumns } from "./columns";
import { getParameterSuggestions } from "../../utils/urlParser";
import { useRequestPageStore } from "@/store/requestPage";

interface ParamsTabContentProps
  extends IDockviewPanelProps<{
    node?: TreeCollectionNode;
    treeId: string;
    iconType: EntryKind;
    someRandomString: string;
  }> {}

export const ParamsTabContent = (_props: ParamsTabContentProps) => {
  const { requestData } = useRequestPageStore();

  // Convert URL parameters to table format with stable IDs for real-time updates
  const convertUrlParamsToTableData = (
    params: Array<{ key: string; value: string }>,
    type: "path" | "query"
  ): ParameterData[] => {
    return params.map((param, index) => {
      const suggestions = getParameterSuggestions(param.key || "param");
      const paramKey = param.key || "";
      const paramValue = param.value || "";

      return {
        order: index + 1,
        id: `${type}-${index}-${Date.now()}`,
        key: paramKey,
        value: paramValue,
        type: paramKey ? suggestions.type : "string",
        description: paramKey ? suggestions.description : `${type === "path" ? "Path" : "Query"} parameter`,
        global_value: "",
        local_value: 0,
        properties: { disabled: false },
      };
    });
  };

  const queryParams = convertUrlParamsToTableData(requestData.url.query_params, "query");
  const pathParams = convertUrlParamsToTableData(requestData.url.path_params, "path");

  return (
    <div className="mt-4">
      {/* Query Params */}
      <div className="mb-6">
        <h3 className="mb-3 text-sm font-medium">Query Params</h3>
        <DataTable key={`query-${requestData.url.raw}`} columns={paramColumns} data={queryParams} />
      </div>

      {/* Path Params */}
      <div>
        <h3 className="mb-3 text-sm font-medium">Path Params</h3>
        <DataTable key={`path-${requestData.url.raw}`} columns={paramColumns} data={pathParams} />
      </div>
    </div>
  );
};
