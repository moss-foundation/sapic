import { useState } from "react";
import { createColumnHelper, CellContext, Row } from "@tanstack/react-table";
import { CheckedState } from "@radix-ui/react-checkbox";
import { Icon } from "@/lib/ui";
import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { EntryKind } from "@repo/moss-collection";
import { IDockviewPanelProps } from "@repo/moss-tabs";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { DataTable, TestData } from "@/components/Table";
import { InputTemplating } from "@/components/InputTemplating";
import { ActionMenu } from "@/components";
import { cn } from "@/utils";

// Extended cell context interface to include custom properties
interface ExtendedCellContext<TData, TValue> extends CellContext<TData, TValue> {
  focusOnMount?: boolean;
}

// Column helper for parameter tables using TestData
const columnHelper = createColumnHelper<TestData>();

// Default input cell component for parameters
const ParamInputCell = ({ info }: { info: ExtendedCellContext<TestData, string> }) => {
  const [value, setValue] = useState(info.getValue());
  const isDisabled = info.row.original.properties.disabled;

  const onBlur = () => {
    info.table.options.meta?.updateData(info.row.index, info.column.id, value);
  };

  return (
    <input
      className={`w-full truncate border-none bg-transparent px-2 py-1.5 placeholder-(--moss-requestpage-placeholder-color) focus:outline-1 focus:outline-blue-500 ${
        isDisabled ? "text-(--moss-requestpage-text-disabled)" : "text-(--moss-primary-text)"
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
const ValueTemplateCell = ({ info }: { info: ExtendedCellContext<TestData, string> }) => {
  const [value, setValue] = useState(info.getValue());
  const isDisabled = info.row.original.properties.disabled;

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setValue(e.target.value);
  };

  const handleTemplateChange = (newValue: string) => {
    setValue(newValue);
    info.table.options.meta?.updateData(info.row.index, info.column.id, newValue);
  };

  const handleBlur = () => {
    info.table.options.meta?.updateData(info.row.index, info.column.id, value);
  };

  // Check if value contains template variables
  const hasTemplateVariables = /\{\{[^}]+\}\}/.test(value);

  return (
    <div className={`w-full`}>
      <InputTemplating
        value={value}
        onChange={handleChange}
        onTemplateChange={handleTemplateChange}
        onBlur={handleBlur}
        placeholder={info.column.id}
        size="sm"
        className={`w-full rounded-none border-none placeholder-(--moss-requestpage-placeholder-color) focus:outline-none ${
          isDisabled
            ? "text-(--moss-requestpage-text-disabled)"
            : hasTemplateVariables
              ? "" // Let InputTemplating handle template variable colors
              : "text-(--moss-primary-text)"
        }`}
        autoFocus={info.focusOnMount}
      />
    </div>
  );
};

// Type selector cell component
const TypeSelectCell = ({ info }: { info: ExtendedCellContext<TestData, string> }) => {
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
const ActionsCell = ({}: { row: Row<TestData> }) => {
  return (
    <div className="flex items-center gap-0.5">
      <button>
        <Icon icon="AddToVcs" className="size-4" />
      </button>
      <button>
        <Icon icon="ConfigMap" className="size-4" />
      </button>
      <button>
        <Icon icon="RemoveCircle" className="size-4" />
      </button>
    </div>
  );
};

// Enabled checkbox cell component
const EnabledCheckboxCell = ({ row, table }: { row: Row<TestData>; table: any }) => {
  const enabled = !row.original.properties.disabled;

  const handleCheckedChange = (checked: CheckedState) => {
    const isChecked = checked === true;

    // Update the data using the table's update mechanism
    const updatedProperties = {
      ...row.original.properties,
      disabled: !isChecked,
    };

    // Use the table's meta updateData function to trigger re-renders
    table.options.meta?.updateData(row.index, "properties", updatedProperties);
  };

  return (
    <div className="flex items-center">
      <CheckboxWithLabel checked={enabled} onCheckedChange={handleCheckedChange} />
    </div>
  );
};

// Column definitions for parameter tables
const paramColumns = [
  columnHelper.display({
    id: "enabled",
    header: "",
    cell: ({ row, table }) => <EnabledCheckboxCell row={row} table={table} />,
    enableSorting: false,
    enableResizing: false,
    size: 40,
  }),
  columnHelper.accessor("key", {
    header: () => "Key",
    cell: (info) => {
      const isDisabled = info.row.original.properties.disabled;
      return (
        <div className="flex items-center">
          <span
            className={`px-2 py-1.5 text-sm ${
              isDisabled ? "text-(--moss-requestpage-text-disabled)" : "text-(--moss-primary-text)"
            }`}
          >
            {info.getValue()}
          </span>
        </div>
      );
    },
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
