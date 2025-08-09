import { createColumnHelper } from "@tanstack/react-table";
import { ParameterData } from "@/components/Table";
import { DefaultInputCell } from "@/components/Table/ui/DefaultCellInput";
import { EnabledCheckboxCell, TypeSelectCell, ActionsCell, EnabledHeaderCheckbox } from "./cells";

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
    header: () => <span className="font-medium">Key</span>,
    cell: (info) => <DefaultInputCell info={info} enableTemplating={true} />,
    minSize: 200,
  }),
  columnHelper.accessor("value", {
    header: () => <span className="font-medium">Value</span>,
    cell: (info) => <DefaultInputCell info={info} enableTemplating={true} />,
    minSize: 150,
  }),
  columnHelper.accessor("type", {
    header: () => <span className="font-medium">Type</span>,
    cell: (info) => <TypeSelectCell info={info} />,
    minSize: 100,
  }),
  columnHelper.accessor("description", {
    header: () => <span className="font-medium">Description</span>,
    cell: (info) => <DefaultInputCell info={info} enableTemplating={false} />,
    meta: {
      isGrow: true,
    },
    minSize: 200,
  }),
  columnHelper.display({
    id: "actions",
    header: () => <span className="font-medium">Actions</span>,
    cell: ({ row }) => <ActionsCell row={row} />,
    enableSorting: false,
    enableResizing: false,
    size: 100,
  }),
];
