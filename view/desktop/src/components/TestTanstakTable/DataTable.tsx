import { useState } from "react";

import { Scrollbar } from "@/lib/ui";
import { ColumnDef, flexRender, getCoreRowModel, getSortedRowModel, useReactTable } from "@tanstack/react-table";

import * as TableBody from "./TableBody";
import * as TableHead from "./TableHead";

interface DataTableProps<TData, TValue> {
  columns: ColumnDef<TData, TValue>[];
  data: TData[];
}

export function DataTable<TData, TValue>({ columns, data }: DataTableProps<TData, TValue>) {
  const [rowSelection, setRowSelection] = useState({});
  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
    onRowSelectionChange: setRowSelection,
    getSortedRowModel: getSortedRowModel(),
    enableColumnResizing: true,
    columnResizeMode: "onChange",
    state: {
      rowSelection,
    },
  });

  return (
    <Scrollbar className="w-full overflow-hidden rounded border-1 border-[#E0E0E0]">
      <table className="table-fixed" style={{ width: table.getTotalSize() }}>
        <TableHead.Head>
          {table.getHeaderGroups().map((headerGroup) => (
            <tr key={headerGroup.id}>
              {headerGroup.headers.map((header) => {
                return (
                  <th
                    key={header.id}
                    className="relative bg-[#F5F5F5] capitalize"
                    style={{ width: header.column.getSize() }}
                  >
                    <span className="relative cursor-pointer" onClick={header.column.getToggleSortingHandler()}>
                      {header.isPlaceholder ? null : flexRender(header.column.columnDef.header, header.getContext())}
                      {{
                        asc: " 🔼",
                        desc: " 🔽",
                      }[header.column.getIsSorted() as string] ?? null}
                    </span>

                    {header.column.getCanResize() && (
                      <div
                        onClick={(e) => e.stopPropagation()}
                        className="absolute top-0 -right-[3px] h-full w-[6px] cursor-col-resize bg-blue-600 transition-colors duration-200 select-none"
                        onMouseDown={header.getResizeHandler()}
                      />
                    )}
                  </th>
                );
              })}
            </tr>
          ))}
        </TableHead.Head>
        <TableBody.Body>
          {table.getRowModel().rows?.length ? (
            table.getRowModel().rows.map((row) => (
              <TableBody.Row key={row.id} data-state={row.getIsSelected() && "selected"}>
                {row.getVisibleCells().map((cell) => {
                  return (
                    <td style={{ width: cell.column.getSize() !== 150 ? cell.column.getSize() : "auto" }} key={cell.id}>
                      <span className="flex items-center justify-center truncate">
                        {flexRender(cell.column.columnDef.cell, cell.getContext())}
                      </span>
                    </td>
                  );
                })}
              </TableBody.Row>
            ))
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
  );
}
