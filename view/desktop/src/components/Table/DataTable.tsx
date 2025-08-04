import { useEffect, useId, useMemo, useRef, useState } from "react";

import { Scrollbar } from "@/lib/ui";
import {
  getCoreRowModel,
  getSortedRowModel,
  SortingState,
  useReactTable,
  VisibilityState,
} from "@tanstack/react-table";

import { detectValueType, getParameterSuggestions } from "@/pages/RequestPage/utils/urlParser";

import { useAdjustColumnsWithoutSizes } from "./hooks/useAdjustColumnsWithoutSizes";
import { useTableDragAndDrop } from "./hooks/useTableDragAndDrop";
import { DataTableProps, ParameterData } from "./types";
import { DefaultCell } from "./ui/DefaultCell";
import DefaultHeader from "./ui/DefaultHeader";
import { DefaultRow } from "./ui/DefaultRow";
import { DefaultAddNewRowForm } from "./ui/DefaultRowForm";
import { NoDataRow } from "./ui/NoDataRow";

export function DataTable({
  columns,
  data: initialData,
  onTableApiSet,
  onDataChange,
  tableType = "ActionsTable",
}: DataTableProps<ParameterData>) {
  const tableId = useId();

  const [data, setData] = useState<ParameterData[]>(initialData);
  const [rowSelection, setRowSelection] = useState({});
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const [focusInputType, setFocusInputType] = useState<string | null>(null);

  const hasActiveInput = useRef(false);

  useEffect(() => {
    if (hasActiveInput.current) {
      return;
    }

    isUpdatingFromProps.current = true;
    setData(initialData);
  }, [initialData]);

  const isInitialLoad = useRef(true);
  const isUpdatingFromProps = useRef(false);
  const onDataChangeRef = useRef(onDataChange);

  useEffect(() => {
    onDataChangeRef.current = onDataChange;
  }, [onDataChange]);

  useEffect(() => {
    if (isInitialLoad.current) {
      isInitialLoad.current = false;
      return;
    }

    if (isUpdatingFromProps.current) {
      isUpdatingFromProps.current = false;
      return;
    }

    onDataChangeRef.current?.(data);
  }, [data]);

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
      tableId,
      tableType,
      updateData: (rowIndex, columnId, value) => {
        // Mark that user is actively editing
        hasActiveInput.current = true;

        setTimeout(() => {
          hasActiveInput.current = false;
        }, 1500);

        setFocusInputType(null);
        setData((old) => {
          const newData = old.map((row, index) => {
            if (index === rowIndex) {
              const currentRow = old[rowIndex]!;
              let updatedRow = {
                ...currentRow,
                [columnId]: value,
              };

              const shouldApplyAutomation = columnId === "key" || columnId === "value";

              if (shouldApplyAutomation) {
                if (columnId === "key") {
                  const keyValue = value as string;
                  if (keyValue && keyValue.trim() !== "") {
                    const suggestions = getParameterSuggestions(keyValue);
                    updatedRow = {
                      ...updatedRow,
                      type: suggestions.type,
                      description: suggestions.description,
                    };
                  }
                } else if (columnId === "value") {
                  const valueStr = value as string;
                  const detectedType = detectValueType(valueStr);
                  updatedRow = {
                    ...updatedRow,
                    type: detectedType,
                  };
                }
              }

              return updatedRow;
            }
            return row;
          });

          return newData;
        });
      },
    },
  });

  const columnSizeVars = useMemo(() => {
    const headers = table.getFlatHeaders();
    const colSizes: { [key: string]: number } = {};
    for (let i = 0; i < headers.length; i++) {
      const header = headers[i]!;
      colSizes[`--header-${header.id}-size`] = header.getSize();
      colSizes[`--col-${header.column.id}-size`] = header.column.getSize();
    }
    return colSizes;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [table.getState().columnSizingInfo, table.getState().columnSizing]);

  const tableContainerRef = useRef<HTMLDivElement>(null);
  const tableHeight = tableContainerRef.current?.clientHeight;

  useTableDragAndDrop({ table, tableId, setSorting, setData, setRowSelection });
  useAdjustColumnsWithoutSizes({ table, tableContainerRef });

  useEffect(() => {
    if (table) onTableApiSet?.(table);
  }, [onTableApiSet, table]);

  const addNewRowAtTheEnd = (e: React.ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();

    const value = e.target.value.trim();
    if (!value) return;

    const columnId = e.target.placeholder;
    const newId = Math.random().toString(36).substring(2, 15);

    setFocusInputType(columnId);

    const newRow: ParameterData = {
      order: data.length + 1,
      id: newId,
      key: columnId === "key" ? value : "",
      value: columnId === "value" ? value : "",
      type: columnId === "type" ? value : "string",
      description: columnId === "description" ? value : "",
      global_value: columnId === "global_value" ? value : "",
      local_value: columnId === "local_value" ? Number(value) || 0 : 0,
      properties: { disabled: false },
    };

    setData((prev) => [...prev, newRow]);
    setRowSelection((prev) => ({ ...prev, [newId]: true }));
  };

  const handleAddNewRowFromDivider = (index: number) => {
    setData((prev) => {
      const newRow: ParameterData = {
        order: index,
        id: Math.random().toString(36).substring(2, 15),
        key: "",
        value: "",
        type: "string",
        description: "",
        global_value: "",
        local_value: 0,
        properties: { disabled: false },
      };

      return [...prev.slice(0, index), newRow, ...prev.slice(index)].map((row, index) => ({
        ...row,
        order: index + 1,
      }));
    });
  };

  return (
    <Scrollbar className="relative -ml-2 w-[calc(100%+8px)] pl-2">
      <div ref={tableContainerRef} className="w-[calc(100%-1px)]">
        <div
          role="table"
          className="rounded border border-(--moss-border-color)"
          style={{ ...columnSizeVars, width: table.getTotalSize() }}
        >
          <div role="rowgroup">
            {table.getHeaderGroups().map((headerGroup) => (
              <div role="row" key={headerGroup.id} className="background-(--moss-table-header-bg) flex">
                {headerGroup.headers.map((header) => (
                  <DefaultHeader tableHeight={tableHeight} key={header.id} header={header} />
                ))}
              </div>
            ))}
          </div>

          <div role="rowgroup">
            {table.getRowModel().rows?.length ? (
              <>
                {table.getRowModel().rows.map((row) => (
                  <DefaultRow
                    onAddNewRow={() => handleAddNewRowFromDivider(row.index)}
                    table={table}
                    row={row}
                    key={row.id}
                  >
                    {row.getVisibleCells().map((cell) => (
                      <DefaultCell
                        key={cell.id}
                        cell={cell}
                        focusOnMount={focusInputType === cell.column.id && cell.row.index === data.length - 1}
                      />
                    ))}
                  </DefaultRow>
                ))}
                <DefaultAddNewRowForm
                  table={table}
                  key={`${tableId}-AddNewRowForm-${data.length}`}
                  onInput={addNewRowAtTheEnd}
                />
              </>
            ) : (
              <NoDataRow setData={setData} tableId={tableId} table={table} />
            )}
          </div>
        </div>
      </div>
    </Scrollbar>
  );
}
