import { createColumnHelper } from "@tanstack/react-table";
import { ParameterData } from "@/components/Table";
import {
  EnabledCheckboxCell,
  TemplateInputCell,
  TypeSelectCell,
  ParamInputCell,
  ActionsCell,
  EnabledHeaderCheckbox,
} from "./cells";

const columnHelper = createColumnHelper<ParameterData>();

export const paramColumns = [
  columnHelper.display({
    id: "enabled",
    header: ({ table }) => <EnabledHeaderCheckbox table={table} />,
    cell: ({ row, table }) => <EnabledCheckboxCell row={row} table={table} />,
    enableSorting: false,
    enableResizing: false,
    size: 40,
  }),
  columnHelper.accessor("key", {
    header: () => "Key",
    cell: (info) => <TemplateInputCell info={info} />,
    minSize: 200,
  }),
  columnHelper.accessor("value", {
    header: () => "Value",
    cell: (info) => <TemplateInputCell info={info} />,
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
