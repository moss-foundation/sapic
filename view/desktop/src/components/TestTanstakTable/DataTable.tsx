import { useId, useState } from "react";

import { Scrollbar } from "@/lib/ui";
import { cn } from "@/utils";
import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  getSortedRowModel,
  SortingState,
  useReactTable,
} from "@tanstack/react-table";

import * as TableBody from "./TableBody";
import * as TableHead from "./TableHead";
import DefaultHeader from "./ui/DefaultHeader";

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

    state: {
      rowSelection,
      sorting,
    },
    meta: {
      id: useId(),
    },
  });

  return (
    <div id={`table-wrapper-test`} className="relative">
      <Scrollbar
        className="w-full rounded border border-[#E0E0E0]"
        style={{
          overflow: "clip",
          overflowClipMargin: 8,
        }}
      >
        <table className="table-fixed border-collapse" style={{ width: table.getTotalSize() }}>
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
                    <TableBody.Row key={row.id} data-state={row.getIsSelected() && "selected"}>
                      {row.getVisibleCells().map((cell) => {
                        return (
                          <td
                            className={cn("border-1 border-l-0 border-[#E0E0E0] px-2 py-1.5")}
                            style={{ width: cell.column.getSize() !== 150 ? cell.column.getSize() : "auto" }}
                            key={cell.id}
                          >
                            {flexRender(cell.column.columnDef.cell, cell.getContext())}
                          </td>
                        );
                      })}
                    </TableBody.Row>
                    {isLastRow && (
                      <tr>
                        {row.getVisibleCells().map((cell) => {
                          console.log({ cell });

                          if (cell.column.id === "actions" || cell.column.id === "checkbox") {
                            return (
                              <td
                                className={cn("border-1 border-b-0 border-l-0 border-[#E0E0E0] px-2 py-1.5")}
                                style={{ width: cell.column.getSize() !== 150 ? cell.column.getSize() : "auto" }}
                                key={cell.id}
                              />
                            );
                          }

                          return (
                            <td
                              className={cn("border-1 border-b-0 border-l-0 border-[#E0E0E0] px-2 py-1.5")}
                              style={{ width: cell.column.getSize() !== 150 ? cell.column.getSize() : "auto" }}
                              key={cell.id}
                            >
                              <input placeholder={cell.column.id} className="w-full outline-0" />
                            </td>
                          );
                        })}
                      </tr>
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
      </Scrollbar>
    </div>
  );
}
