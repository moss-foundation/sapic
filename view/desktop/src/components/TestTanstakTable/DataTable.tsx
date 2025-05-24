import { useState } from "react";

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
    state: {
      rowSelection,
    },
  });

  return (
    <div className="rounded border-1 border-[#E0E0E0]">
      <table>
        <TableHead.Head>
          {table.getHeaderGroups().map((headerGroup) => (
            <TableHead.Row key={headerGroup.id}>
              {headerGroup.headers.map((header) => {
                return (
                  <td key={header.id}>
                    {header.isPlaceholder ? null : flexRender(header.column.columnDef.header, header.getContext())}
                  </td>
                );
              })}
            </TableHead.Row>
          ))}
        </TableHead.Head>
        <TableBody.Body>
          {table.getRowModel().rows?.length ? (
            table.getRowModel().rows.map((row) => (
              <TableBody.Row key={row.id} data-state={row.getIsSelected() && "selected"}>
                {row.getVisibleCells().map((cell) => (
                  <TableBody.Cell key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </TableBody.Cell>
                ))}
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
    </div>
  );
}
