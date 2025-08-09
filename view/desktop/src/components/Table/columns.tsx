import { createColumnHelper } from "@tanstack/react-table";

import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { ParameterData } from "./types";
import { DefaultInputCell } from "./ui/DefaultCellInput";
import { Icon } from "@/lib/ui";

const columnHelper = createColumnHelper<ParameterData>();

export const columns = [
  columnHelper.display({
    id: "checkbox",
    header: ({ table }) => (
      <CheckboxWithLabel
        checked={table.getIsAllPageRowsSelected()}
        onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
      />
    ),
    cell: ({ row }) => (
      <CheckboxWithLabel
        disabled={!row.getCanSelect()}
        checked={row.getIsSelected()}
        onCheckedChange={row.getToggleSelectedHandler()}
      />
    ),
    enableSorting: false,
    enableResizing: false,
    size: 40,
  }),
  columnHelper.accessor("key", {
    header: () => "key",
    cell: (info) => <DefaultInputCell info={info} />,
    minSize: 60,
  }),
  columnHelper.accessor("value", {
    header: () => "value",
    cell: (info) => <DefaultInputCell info={info} />,
    minSize: 100,
  }),
  columnHelper.accessor("type", {
    header: () => "type",
    cell: (info) => <DefaultInputCell info={info} />,
    minSize: 100,
  }),
  columnHelper.accessor("description", {
    header: () => "description",
    cell: (info) => <DefaultInputCell info={info} />,
    meta: {
      isGrow: true,
    },
    minSize: 100,
  }),
  columnHelper.accessor("global_value", {
    header: () => "Global value",
    cell: (info) => <DefaultInputCell info={info} />,
    minSize: 100,
  }),
  columnHelper.accessor("local_value", {
    header: () => "Local value",
    cell: (info) => <DefaultInputCell info={info} />,
    minSize: 100,
  }),
  columnHelper.display({
    id: "actions",
    header: () => "Actions",
    cell: () => (
      <div className="flex items-center justify-center gap-1">
        <button className="flex size-5.5 cursor-pointer items-center justify-center rounded hover:bg-[#E0E0E0]">
          <Icon icon="AddToVcs" />
        </button>

        <button className="flex size-5.5 cursor-pointer items-center justify-center rounded hover:bg-[#E0E0E0]">
          <Icon icon="RemoveCircle" />
        </button>
        <button className="flex size-5.5 cursor-pointer items-center justify-center rounded hover:bg-[#E0E0E0]">
          <Icon icon="ConfigMap" />
        </button>
      </div>
    ),
    enableSorting: false,
    enableResizing: false,
    size: 90,
  }),
];
