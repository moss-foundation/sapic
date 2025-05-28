import { useId, useLayoutEffect, useRef, useState } from "react";

import { Scrollbar } from "@/lib/ui";
import { cn } from "@/utils";
import { ColumnDef, getCoreRowModel, getSortedRowModel, SortingState, useReactTable } from "@tanstack/react-table";

import { calculateTableSizing } from "./calculateTableSizing";
import * as TableBody from "./TableBody";
import * as TableHead from "./TableHead";
import { DefaultCell } from "./ui/DefaultCell";
import DefaultHeader from "./ui/DefaultHeader";
import { DefaultRow } from "./ui/DefaultRow";

interface DataTableProps<TData, TValue> {
  columns: ColumnDef<TData, TValue>[];
  data: TData[];
}

export function DataTable<TData, TValue>({ columns, data: initialData }: DataTableProps<TData, TValue>) {
  const [data, setData] = useState<TData[]>(initialData);

  const [rowSelection, setRowSelection] = useState({});
  const [sorting, setSorting] = useState<SortingState>([]);
  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
    onRowSelectionChange: setRowSelection,
    getSortedRowModel: getSortedRowModel(),
    onSortingChange: setSorting,
    enableColumnResizing: true,
    columnResizeMode: "onChange",
    enableRowSelection: (row) => !row.original.properties.disabled,
    state: {
      rowSelection,
      sorting,
    },
    meta: {
      id: useId(),
    },
  });

  const tableContainerRef = useRef<HTMLDivElement>(null);
  const headers = table.getFlatHeaders();
  useLayoutEffect(() => {
    if (!tableContainerRef.current) return;
    const resizeObserver = new ResizeObserver((entries) => {
      const entry = entries[0];
      if (entry) {
        const initialColumnSizing = calculateTableSizing(headers, entry.contentRect.width);
        table.setColumnSizing(initialColumnSizing);
      }
    });
    resizeObserver.observe(tableContainerRef.current);
    return () => {
      resizeObserver.disconnect();
    };
  }, [headers, table]);

  return (
    <Scrollbar className="w-full">
      <div className="w-[calc(100%-1px)]" ref={tableContainerRef}>
        <table
          className="w-full table-fixed border-collapse rounded border border-[#E0E0E0]"
          style={{ width: table.getCenterTotalSize() }}
        >
          <TableHead.Head>
            {table.getHeaderGroups().map((headerGroup) => (
              <tr key={headerGroup.id} className="bg-[#F5F5F5]">
                {headerGroup.headers.map((header) => (
                  <DefaultHeader key={header.id} header={header} />
                ))}
              </tr>
            ))}
          </TableHead.Head>
          <TableBody.Body>
            {table.getRowModel().rows?.length ? (
              table.getRowModel().rows.map((row, index) => {
                const isLastRow = index === table.getRowModel().rows.length - 1;

                return (
                  <>
                    <DefaultRow row={row} key={row.id}>
                      {row.getVisibleCells().map((cell) => {
                        return <DefaultCell key={cell.id} cell={cell} />;
                      })}
                    </DefaultRow>

                    {isLastRow && (
                      <DefaultRow row={row} key={"AddNewRowForm"} disableDnd>
                        {row.getVisibleCells().map((cell) => {
                          if (cell.column.id === "actions" || cell.column.id === "checkbox") {
                            return (
                              <td
                                key={cell.id}
                                className={cn("border-1 border-b-0 border-l-0 border-[#E0E0E0] px-2 py-1.5")}
                                style={{ width: cell.column.getSize() !== 150 ? cell.column.getSize() : "auto" }}
                              />
                            );
                          }

                          return (
                            <td
                              key={cell.id}
                              className={cn("border-1 border-b-0 border-l-0 border-[#E0E0E0] px-2 py-1.5")}
                              style={{ width: cell.column.getSize() !== 150 ? cell.column.getSize() : "auto" }}
                            >
                              <input placeholder={cell.column.id} className="w-full outline-0" />
                            </td>
                          );
                        })}
                      </DefaultRow>
                    )}
                  </>
                );
              })
            ) : (
              <TableBody.Row>
                <TableBody.Cell colSpan={columns.length} className="h-24 text-center">
                  No results.
                </TableBody.Cell>
              </TableBody.Row>
            )}
          </TableBody.Body>
        </table>
      </div>
    </Scrollbar>
  );
}
