import { useEffect, useId, useLayoutEffect, useRef, useState } from "react";

import { Scrollbar } from "@/lib/ui";
import { cn } from "@/utils";
import { swapListByIndexWithEdge } from "@/utils/swapListByIndexWithEdge";
import { extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { dropTargetForElements, monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import {
  ColumnDef,
  getCoreRowModel,
  getSortedRowModel,
  RowData,
  SortingState,
  useReactTable,
} from "@tanstack/react-table";

import { calculateTableSizing } from "./calculateTableSizing";
import * as TableBody from "./TableBody";
import * as TableHead from "./TableHead";
import { DefaultCell } from "./ui/DefaultCell";
import DefaultHeader from "./ui/DefaultHeader";
import { DefaultRow } from "./ui/DefaultRow";

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
  }
}

interface TestData {
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

interface DnDRowData {
  type: "TableRow";
  data: {
    tableId: string;
    row: TestData;
  };
}

export function DataTable<TValue>({ columns, data: initialData }: DataTableProps<TestData, TValue>) {
  const tableId = useId();
  const [data, setData] = useState<TestData[]>(initialData);

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
    enableRowSelection: true,
    columnResizeMode: "onChange",
    getRowId: (row) => row.key,
    state: {
      rowSelection,
      sorting,
    },
    meta: {
      id: tableId,
    },
  });

  const tableContainerRef = useRef<HTMLDivElement>(null);
  const tableHeight = tableContainerRef.current?.clientHeight;
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

  useEffect(() => {
    return monitorForElements({
      canMonitor: ({ source }) => {
        return source.data.type === "TableRow";
      },

      onDrop({ location, source }) {
        if (source.data.type !== "TableRow" || location.current.dropTargets.length === 0) return;

        const sourceTarget = source.data.data as DnDRowData["data"];
        const dropTarget = location.current.dropTargets[0].data.data as DnDRowData["data"];
        const edge = extractClosestEdge(location.current.dropTargets[0].data);

        if (sourceTarget.tableId === dropTarget.tableId) {
          if (dropTarget.tableId === tableId && sourceTarget.tableId === tableId) {
            setData((prev) => {
              const sourceIndex = prev.findIndex((row) => row.key === sourceTarget.row.key);
              const dropIndex = prev.findIndex((row) => row.key === dropTarget.row.key);

              return swapListByIndexWithEdge(sourceIndex, dropIndex, prev, edge);
            });
          }
          return;
        }

        if (sourceTarget.tableId === tableId) {
          setData((prev) => {
            return [...prev].filter((row) => row.key !== sourceTarget.row.key);
          });
          return;
        }

        if (dropTarget.tableId === tableId) {
          const edge = extractClosestEdge(location.current.dropTargets[0].data);
          setData((prev) => {
            const dropIndex = prev.findIndex((row) => row.key === dropTarget.row.key);
            const newData = [...prev];

            const insertIndex = edge === "bottom" ? dropIndex + 1 : dropIndex;
            newData.splice(insertIndex, 0, sourceTarget.row);

            return newData;
          });
        }
        return;
      },
    });
  }, [tableId]);

  const handleBlur = (e: React.FocusEvent<HTMLInputElement>) => {
    e.preventDefault();

    const value = e.target.value.trim();
    if (!value) return;

    const columnId = e.target.placeholder;
    const newRow: TestData = {
      key: `rand_key: ${Math.random().toString(36).substring(2, 15)}`,
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

  return (
    <Scrollbar className="relative -ml-2 w-[calc(100%+8px)] pl-2">
      <div className="w-[calc(100%-1px)]" ref={tableContainerRef}>
        <table
          className="w-full table-fixed border-collapse rounded border border-[#E0E0E0]"
          style={{ width: table.getCenterTotalSize() }}
        >
          <TableHead.Head>
            {table.getHeaderGroups().map((headerGroup) => (
              <tr key={headerGroup.id} className="bg-[#F5F5F5]">
                {headerGroup.headers.map((header) => (
                  <DefaultHeader tableHeight={tableHeight} key={header.id} header={header} />
                ))}
              </tr>
            ))}
          </TableHead.Head>
          <TableBody.Body>
            {table.getRowModel().rows?.length ? (
              <>
                {table.getRowModel().rows.map((row) => (
                  <DefaultRow table={table} row={row} key={row.id}>
                    {row.getVisibleCells().map((cell) => (
                      <DefaultCell key={cell.id} cell={cell} />
                    ))}
                  </DefaultRow>
                ))}
                <DefaultRow
                  table={table}
                  row={table.getRowModel().rows[table.getRowModel().rows.length - 1]}
                  key="AddNewRowForm"
                  disableDnd
                >
                  {table
                    .getRowModel()
                    .rows[table.getRowModel().rows.length - 1].getVisibleCells()
                    .map((cell) => {
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
                          <input
                            form={`${tableId}-AddNewRowForm`}
                            placeholder={cell.column.id}
                            className="w-full outline-0"
                            onBlur={handleBlur}
                          />
                        </td>
                      );
                    })}
                </DefaultRow>
              </>
            ) : (
              <EmptyRow colSpan={columns.length} setData={setData} tableId={tableId} />
            )}
          </TableBody.Body>
        </table>
        <form onSubmit={(e) => e.preventDefault()} id={`${tableId}-AddNewRowForm`} className="sr-only" />
      </div>
    </Scrollbar>
  );
}

const EmptyRow = ({
  colSpan,
  setData,
  tableId,
}: {
  colSpan: number;
  setData: (data: TestData[]) => void;
  tableId: string;
}) => {
  const ref = useRef<HTMLTableRowElement>(null);
  const [isDraggedOver, setIsDraggedOver] = useState(false);

  useEffect(() => {
    const element = ref.current;
    if (!element) return;

    return combine(
      monitorForElements({
        canMonitor: ({ source }) => {
          return source.data.type === "TableRow";
        },
        onDrop({ location, source }) {
          if (source.data.type !== "TableRow" || location.current.dropTargets.length === 0) return;

          const sourceTarget = source.data.data as DnDRowData["data"];
          const dropTarget = location.current.dropTargets[0].data.data as DnDRowData["data"];

          if (dropTarget.tableId === tableId) {
            setData([sourceTarget.row]);
          }
        },
      }),
      dropTargetForElements({
        element,
        getDropEffect: () => "copy",
        getData: () => {
          return { type: "TableRowNoResults", data: { tableId } };
        },
        onDragEnter() {
          setIsDraggedOver(true);
        },
        onDragLeave() {
          setIsDraggedOver(false);
        },
        canDrop: ({ source }) => {
          return source.data.type === "TableRow";
        },
      })
    );
  }, [setData, tableId]);

  return (
    <tr ref={ref} key={`empty-row-${tableId}`}>
      <TableBody.Cell colSpan={colSpan} className={cn("h-24 text-center", isDraggedOver && "bg-blue-400")}>
        No results.
      </TableBody.Cell>
    </tr>
  );
};
