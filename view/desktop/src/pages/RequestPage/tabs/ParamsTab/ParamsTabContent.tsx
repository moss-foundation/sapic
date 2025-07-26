import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { EntryKind } from "@repo/moss-collection";
import { IDockviewPanelProps } from "@repo/moss-tabs";
import { DataTable } from "@/components/Table";
import { paramColumns } from "./columns";
import { queryParamsData, pathParamsData } from "./data";

export const ParamsTabContent = ({}: IDockviewPanelProps<{
  node?: TreeCollectionNode;
  treeId: string;
  iconType: EntryKind;
  someRandomString: string;
}>) => {
  return (
    <div className="mt-4">
      {/* Query Params */}
      <div className="mb-6">
        <h3 className="mb-3 text-sm font-medium text-gray-900">Query Params</h3>
        <DataTable columns={paramColumns} data={queryParamsData} />
      </div>

      {/* Path Params */}
      <div>
        <h3 className="mb-3 text-sm font-medium text-gray-900">Path Params</h3>
        <DataTable columns={paramColumns} data={pathParamsData} />
      </div>
    </div>
  );
};
