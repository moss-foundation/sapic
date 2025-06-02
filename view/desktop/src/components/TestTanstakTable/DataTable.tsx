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
  Table,
  useReactTable,
  VisibilityState,
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
    tableType: "ActionsTable";
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

  useEffect(() => {
    console.log(columnVisibility);
  }, [columnVisibility]);

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

  useEffect(() => {
    if (table) {
      onTableApiSet?.(table);
    }
  }, [onTableApiSet, table]);

  const tableContainerRef = useRef<HTMLDivElement>(null);
  const tableHeight = tableContainerRef.current?.clientHeight;

  const headers = table.getFlatHeaders();

  //this should only be called on mount
  useLayoutEffect(() => {
    if (tableContainerRef.current) {
      const initialColumnSizing = calculateTableSizing(headers, tableContainerRef.current?.clientWidth);
      table.setColumnSizing(initialColumnSizing);
    }
  }, []);

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

        const flatRows = table.getRowModel().flatRows.map((row) => row.original);

        if (sourceTarget.tableId === dropTarget.tableId) {
          if (dropTarget.tableId === tableId && sourceTarget.tableId === tableId) {
            setSorting([]);
            const sourceIndex = flatRows.findIndex((row) => row.key === sourceTarget.row.key);
            const dropIndex = flatRows.findIndex((row) => row.key === dropTarget.row.key);

            const newData = swapListByIndexWithEdge(sourceIndex, dropIndex, flatRows, edge);
            setData(newData);
          }
          return;
        }

        if (sourceTarget.tableId === tableId) {
          setSorting([]);

          setData((prev) => {
            return [...prev].filter((row) => row.key !== sourceTarget.row.key);
          });
          return;
        }

        if (dropTarget.tableId === tableId) {
          setSorting([]);
          const edge = extractClosestEdge(location.current.dropTargets[0].data);

          const dropIndex = flatRows.findIndex((row) => row.key === dropTarget.row.key);
          const newData = [...flatRows];

          const insertIndex = edge === "bottom" ? dropIndex + 1 : dropIndex;
          newData.splice(insertIndex, 0, sourceTarget.row);

          setData(newData);
        }
        return;
      },
    });
  }, [table, tableId]);

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

  const handleAddNewRow = (index: number) => {
    setData((prev) => {
      const newRow: TestData = {
        key: `rand_key: ${Math.random().toString(36).substring(2, 15)}`,
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
          className="border-separate border-spacing-0 rounded border border-[#E0E0E0]"
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
                  <DefaultRow onAddNewRow={() => handleAddNewRow(row.index)} table={table} row={row} key={row.id}>
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
                      const isLastColumn = cell.column.getIsLastColumn();
                      if (cell.column.id === "actions" || cell.column.id === "checkbox") {
                        return (
                          <td
                            key={cell.id}
                            className={cn("border-r border-[#E0E0E0] px-2 py-1.5", isLastColumn && "border-r-0")}
                            style={{ width: cell.column.getSize() !== 150 ? cell.column.getSize() : "auto" }}
                          />
                        );
                      }
                      return (
                        <td
                          key={cell.id}
                          className={cn("border-r border-[#E0E0E0] px-2 py-1.5", isLastColumn && "border-r-0")}
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
      <td colSpan={colSpan} className={cn("h-24 text-center", isDraggedOver && "bg-blue-400")}>
        No results.
      </td>
    </tr>
  );
};
