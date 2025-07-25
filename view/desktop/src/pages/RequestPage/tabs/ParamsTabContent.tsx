import { useState } from "react";
import { createColumnHelper } from "@tanstack/react-table";
import { Icon } from "@/lib/ui";
import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { EntryKind } from "@repo/moss-collection";
import { IDockviewPanelProps } from "@repo/moss-tabs";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { DataTable, TestData } from "@/components/Table";
import { InputTemplating } from "@/components/InputTemplating";
import { ActionMenu } from "@/components";
import { cn } from "@/utils";

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

// Template input cell component for value column with variable highlighting
const ValueTemplateCell = ({ info }: { info: any }) => {
  const [value, setValue] = useState(info.getValue());
  //const isSelected = info.row.getIsSelected();

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setValue(e.target.value);
  };

  const handleTemplateChange = (newValue: string, variables: string[]) => {
    setValue(newValue);
    info.table.options.meta?.updateData(info.row.index, info.column.id, newValue);
  };

  const handleBlur = () => {
    info.table.options.meta?.updateData(info.row.index, info.column.id, value);
  };

  return (
    <div className={`w-full`}>
      <InputTemplating
        value={value}
        onChange={handleChange}
        onTemplateChange={handleTemplateChange}
        onBlur={handleBlur}
        placeholder={info.column.id}
        size="sm"
        className="w-full rounded-none border-none focus:outline-none"
        autoFocus={info.focusOnMount}
      />
    </div>
  );
};

// Type selector cell component
const TypeSelectCell = ({ info }: { info: any }) => {
  const [value, setValue] = useState(info.getValue());
  const DATA_TYPES = ["string", "number", "bool"];

  const handleTypeChange = (newType: string) => {
    setValue(newType);
    info.table.options.meta?.updateData(info.row.index, info.column.id, newType);
  };

  const getTypeColor = (type: string) => {
    switch (type) {
      case "string":
        return "text-(--moss-requestpage-string-color)";
      case "number":
        return "text-(--moss-requestpage-number-color)";
      case "bool":
        return "text-(--moss-requestpage-bool-color)";
      default:
        return "text-gray-500";
    }
  };

  return (
    <ActionMenu.Root>
      <ActionMenu.Trigger asChild>
        <button
          className={cn(
            "flex w-full items-center justify-between px-2 py-1.5 text-sm transition-colors",
            "border-none bg-transparent",
            "focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-(--moss-primary)",
            "data-[state=open]:outline-2 data-[state=open]:-outline-offset-1 data-[state=open]:outline-(--moss-primary)",
            getTypeColor(value)
          )}
        >
          <span>{value}</span>
          <Icon icon="ChevronDown" className="h-3 w-3 cursor-pointer text-(--moss-requestpage-icon-color)" />
        </button>
      </ActionMenu.Trigger>
      <ActionMenu.Content>
        {DATA_TYPES.map((dataType) => (
          <ActionMenu.Item
            key={dataType}
            onClick={() => handleTypeChange(dataType)}
            className={cn(
              getTypeColor(dataType),
              value === dataType && "background-(--moss-secondary-background-hover) font-medium"
            )}
          >
            {dataType}
          </ActionMenu.Item>
        ))}
      </ActionMenu.Content>
    </ActionMenu.Root>
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
    cell: (info) => <ValueTemplateCell info={info} />,
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
