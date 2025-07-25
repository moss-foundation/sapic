import { useState } from "react";
import { createColumnHelper } from "@tanstack/react-table";
import { Icon } from "@/lib/ui";
import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { EntryKind } from "@repo/moss-collection";
import { IDockviewPanelProps } from "@repo/moss-tabs";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { DataTable, TestData } from "@/components/Table";

// Column helper for parameter tables using TestData
const columnHelper = createColumnHelper<TestData>();

// Default input cell component for parameters
const ParamInputCell = ({ info }: { info: any }) => {
  const [value, setValue] = useState(info.getValue());
  const isSelected = info.row.getIsSelected();

  const onBlur = () => {
    info.table.options.meta?.updateData(info.row.index, info.column.id, value);
  };

  return (
    <input
      className={`w-full truncate px-2 py-1.5 focus:outline-1 focus:outline-blue-500 disabled:text-gray-400 ${
        !isSelected ? "opacity-60" : ""
      }`}
      value={value}
      onChange={(e) => setValue(e.target.value)}
      autoFocus={info.focusOnMount}
      onBlur={onBlur}
      placeholder={info.column.id}
    />
  );
};

// Type selector cell component
const TypeSelectCell = ({ info }: { info: any }) => {
  const [value, setValue] = useState(info.getValue());

  const onBlur = () => {
    info.table.options.meta?.updateData(info.row.index, info.column.id, value);
  };

  return (
    <select
      className="w-full border-none bg-transparent px-2 py-1.5 text-sm outline-none focus:outline-1 focus:outline-blue-500"
      value={value}
      onChange={(e) => {
        setValue(e.target.value);
        info.table.options.meta?.updateData(info.row.index, info.column.id, e.target.value);
      }}
      onBlur={onBlur}
    >
      <option value="string">string</option>
      <option value="number">number</option>
      <option value="bool">bool</option>
    </select>
  );
};

// Actions cell component
const ActionsCell = ({ row }: { row: any }) => {
  return (
    <div className="flex items-center gap-1">
      <button className="p-1 text-gray-400 hover:text-gray-600">
        <Icon icon="Add" className="h-3 w-3" />
      </button>
      <button className="p-1 text-gray-400 hover:text-gray-600">
        <Icon icon="Find" className="h-3 w-3" />
      </button>
      <button className="p-1 text-red-400 hover:text-red-600">
        <Icon icon="Delete" className="h-3 w-3" />
      </button>
    </div>
  );
};

// Column definitions for parameter tables
const paramColumns = [
  columnHelper.display({
    id: "enabled",
    header: "",
    cell: ({ row }) => {
      const [enabled, setEnabled] = useState(!row.original.properties.disabled);
      return (
        <div className="flex items-center">
          <CheckboxWithLabel
            checked={enabled}
            onCheckedChange={(checked) => {
              setEnabled(!!checked);
              // Update the data
              row.original.properties.disabled = !checked;
            }}
          />
        </div>
      );
    },
    enableSorting: false,
    enableResizing: false,
    size: 40,
  }),
  columnHelper.accessor("key", {
    header: () => "Key",
    cell: (info) => (
      <div className="flex items-center">
        <span className="text-sm text-gray-900">{info.getValue()}</span>
      </div>
    ),
    minSize: 100,
  }),
  columnHelper.accessor("value", {
    header: () => "Value",
    cell: (info) => <ParamInputCell info={info} />,
    minSize: 150,
  }),
  columnHelper.accessor("type", {
    header: () => "Type",
    cell: (info) => <TypeSelectCell info={info} />,
    minSize: 100,
  }),
  columnHelper.accessor("description", {
    header: () => "Description",
    cell: (info) => <ParamInputCell info={info} />,
    meta: {
      isGrow: true,
    },
    minSize: 200,
  }),
  columnHelper.display({
    id: "actions",
    header: () => "Actions",
    cell: ({ row }) => <ActionsCell row={row} />,
    enableSorting: false,
    enableResizing: false,
    size: 100,
  }),
];

// Sample data for Query Params
const queryParamsData: TestData[] = [
  {
    order: 1,
    id: "1",
    key: "pageToken",
    value: "{{mu_func()}}",
    type: "string",
    description: "An opaque token used to fetch the next page of results.",
    global_value: "",
    local_value: 0,
    properties: { disabled: false },
  },
  {
    order: 2,
    id: "2",
    key: "limit",
    value: "{{defaultLimit}}",
    type: "number",
    description: "Maximum number of results to return in this query.",
    global_value: "",
    local_value: 0,
    properties: { disabled: false },
  },
  {
    order: 3,
    id: "3",
    key: "visibleOnly",
    value: "true",
    type: "bool",
    description: "If true, returns only visible columns for the table. This...",
    global_value: "",
    local_value: 0,
    properties: { disabled: true },
  },
];

// Sample data for Path Params
const pathParamsData: TestData[] = [
  {
    order: 1,
    id: "4",
    key: "docId",
    value: "{{vault::myVariable}}",
    type: "string",
    description: "An opaque token used to fetch the next page of results.",
    global_value: "",
    local_value: 0,
    properties: { disabled: false },
  },
  {
    order: 2,
    id: "5",
    key: "tableIdOrName",
    value: "{{defaultLimit}}",
    type: "number",
    description: "Maximum number of results to return in this query.",
    global_value: "",
    local_value: 0,
    properties: { disabled: false },
  },
];

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
