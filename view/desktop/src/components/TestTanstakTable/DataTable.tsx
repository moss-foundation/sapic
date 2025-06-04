import { useEffect, useId, useRef, useState } from "react";

import { Scrollbar } from "@/lib/ui";
import { cn } from "@/utils";

import {
  ColumnDef,
  getCoreRowModel,
  getSortedRowModel,
  RowData,
  SortingState,
  Table,
  useReactTable,
  VisibilityState,
} from "@tanstack/react-table";

import { useAdjustColumnsWithoutSizes } from "./hooks/useAdjustColumnsWithoutSizes";
import { useTableRowReorder } from "./hooks/useTableRowReorder";
import { DefaultCell } from "./ui/DefaultCell";
import DefaultHeader from "./ui/DefaultHeader";
import { DefaultRow } from "./ui/DefaultRow";
import { NoDataRow } from "./ui/NoDataRow";

interface DataTableProps<TData, TValue> {
  columns: ColumnDef<TData, TValue>[];
  data: TData[];
  meta?: {
    id: string;
    setData: (data: TData[]) => void;
  };
}

declare module "@tanstack/react-table" {
  interface TableMeta<TData extends RowData> {
    id: string;
    tableType: "ActionsTable";
  }
}

export interface TestData {
  id: string;
  key: string;
  value: string;
  type: string;
  description: string;
  global_value: string;
  local_value: number;
  properties: {
    disabled: boolean;
  };
}

export interface DnDRowData {
  type: "TableRow";
  data: {
    tableId: string;
    row: TestData;
  };
}

export function DataTable<TValue>({
  columns,
  data: initialData,
  onTableApiSet,
}: DataTableProps<TestData, TValue> & {
  onTableApiSet: (table: Table<TestData>) => void;
}) {
  const tableId = useId();

  const [data, setData] = useState<TestData[]>(initialData);
  const [rowSelection, setRowSelection] = useState({});
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});

  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
    onRowSelectionChange: setRowSelection,
    getSortedRowModel: getSortedRowModel(),
    onSortingChange: setSorting,
    enableColumnResizing: true,
    enableRowSelection: true,
    columnResizeMode: "onChange",
    getRowId: (row) => row.id,
    onColumnVisibilityChange: setColumnVisibility,
    state: {
      columnVisibility,
      rowSelection,
      sorting,
    },
    meta: {
      id: tableId,
      tableType: "ActionsTable",
    },
  });

  const tableContainerRef = useRef<HTMLDivElement>(null);
  const tableHeight = tableContainerRef.current?.clientHeight;

  useTableRowReorder({ table, tableId, setSorting, setData });

  useAdjustColumnsWithoutSizes({ table, tableContainerRef });

  useEffect(() => {
    if (table) onTableApiSet?.(table);
  }, [onTableApiSet, table]);

  const handleBlur = (e: React.FocusEvent<HTMLInputElement>) => {
    e.preventDefault();

    const value = e.target.value.trim();
    if (!value) return;

    const columnId = e.target.placeholder;
    const newRow: TestData = {
      id: Math.random().toString(36).substring(2, 15),
      key: columnId === "key" ? value : "",
      value: columnId === "value" ? value : "",
      type: columnId === "type" ? value : "",
      description: columnId === "description" ? value : "",
      global_value: columnId === "global_value" ? value : "",
      local_value: columnId === "local_value" ? Number(value) || 0 : 0,
      properties: {
        disabled: false,
      },
    };

    setData((prev) => [...prev, newRow]);
    e.target.value = "";
  };

  const handleAddNewRow = (index: number) => {
    setData((prev) => {
      const newRow: TestData = {
        id: Math.random().toString(36).substring(2, 15),
        key: "",
        value: "",
        type: "",
        description: "",
        global_value: "",
        local_value: 0,
        properties: {
          disabled: false,
        },
      };

      return [...prev.slice(0, index), newRow, ...prev.slice(index)];
    });
  };

  return (
    <Scrollbar className="relative -ml-2 w-[calc(100%+8px)] pl-2">
      <div className="w-[calc(100%-1px)]" ref={tableContainerRef}>
        <table
          className="border-separate border-spacing-0 rounded border border-(--moss-border-color)"
          style={{ width: table.getCenterTotalSize() }}
        >
          <thead>
            {table.getHeaderGroups().map((headerGroup) => (
              <tr key={headerGroup.id} className="bg-(--moss-table-header-bg)">
                {headerGroup.headers.map((header) => (
                  <DefaultHeader tableHeight={tableHeight} key={header.id} header={header} />
                ))}
              </tr>
            ))}
          </thead>
          <tbody>
            {table.getRowModel().rows?.length ? (
              <>
                {table.getRowModel().rows.map((row) => (
                  <DefaultRow onAddNewRow={() => handleAddNewRow(row.index)} table={table} row={row} key={row.id}>
                    {row.getVisibleCells().map((cell) => (
                      <DefaultCell key={cell.id} cell={cell} />
                    ))}
                  </DefaultRow>
                ))}
                <DefaultRow
                  table={table}
                  row={table.getRowModel().rows[table.getRowModel().rows.length - 1]}
                  key={`${tableId}-AddNewRowForm`}
                  disableDnd
                >
                  {table
                    .getRowModel()
                    .rows[table.getRowModel().rows.length - 1].getVisibleCells()
                    .map((cell) => {
                      const isLastColumn = cell.column.getIsLastColumn();
                      if (cell.column.id === "actions" || cell.column.id === "checkbox") {
                        return (
                          <td
                            key={cell.id}
                            className={cn(
                              "border-r border-(--moss-border-color) px-2 py-1.5",
                              isLastColumn && "border-r-0"
                            )}
                            style={{ width: cell.column.getSize() !== 150 ? cell.column.getSize() : "auto" }}
                          />
                        );
                      }
                      return (
                        <td
                          key={cell.id}
                          className={cn(
                            "border-r border-(--moss-border-color) px-2 py-1.5",
                            isLastColumn && "border-r-0"
                          )}
                          style={{ width: cell.column.getSize() !== 150 ? cell.column.getSize() : "auto" }}
                        >
                          <input
                            form={`${tableId}-AddNewRowForm`}
                            placeholder={cell.column.id}
                            className="w-full text-(--moss-table-add-row-form-text) outline-0"
                            onBlur={handleBlur}
                          />
                        </td>
                      );
                    })}
                </DefaultRow>
              </>
            ) : (
              <NoDataRow colSpan={columns.length} setData={setData} tableId={tableId} />
            )}
          </tbody>
        </table>
        <form onSubmit={(e) => e.preventDefault()} id={`${tableId}-AddNewRowForm`} className="sr-only" />
      </div>
    </Scrollbar>
  );
}
