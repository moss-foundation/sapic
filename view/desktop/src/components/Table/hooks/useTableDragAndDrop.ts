import { Dispatch, SetStateAction, useEffect } from "react";

import { refreshOrders } from "@/utils/refreshOrders";
import { swapListByIndexWithEdge } from "@/utils/swapListByIndexWithEdge";
import { extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { RowSelectionState, SortingState, Table } from "@tanstack/react-table";

import { ParameterData } from "../types";
import { getTableRowData } from "../utils";

interface UseTableDragAndDropProps {
  table: Table<ParameterData>;
  tableId: string;
  setSorting: (sorting: SortingState) => void;
  setData: Dispatch<SetStateAction<ParameterData[]>>;
  setRowSelection: Dispatch<SetStateAction<RowSelectionState>>;
}

export const useTableDragAndDrop = ({
  table,
  tableId,
  setSorting,
  setData,
  setRowSelection,
}: UseTableDragAndDropProps) => {
  useEffect(() => {
    return monitorForElements({
      canMonitor: ({ source }) => {
        if (source.data.type !== "TableRow") {
          return false;
        }

        const sourceTarget = getTableRowData(source);

        if (sourceTarget.tableType !== table.options.meta?.tableType) {
          return false;
        }

        return true;
      },

      onDrop({ location, source }) {
        if (source.data.type !== "TableRow" || location.current.dropTargets.length === 0) return;

        const sourceTarget = getTableRowData(source);
        const dropTarget = getTableRowData(location);

        const edge = extractClosestEdge(location.current.dropTargets[0].data);

        const flatRows = table.getRowModel().flatRows.map((row) => row.original);

        if (!sourceTarget || !dropTarget) {
          return;
        }

        if (sourceTarget.tableId === dropTarget.tableId) {
          if (dropTarget.tableId === tableId && sourceTarget.tableId === tableId) {
            setSorting([]);

            const sourceIndex = flatRows.findIndex((row) => row.id === sourceTarget.row.id);
            const dropIndex = flatRows.findIndex((row) => row.id === dropTarget.row.id);

            const newData = swapListByIndexWithEdge(sourceIndex, dropIndex, flatRows, edge);

            setData(refreshOrders(newData));
          }

          return;
        }

        if (sourceTarget.tableId === tableId) {
          setData((prev) => refreshOrders([...prev].filter((row) => row.id !== sourceTarget.row.id)));

          if (sourceTarget.isSelected) {
            setRowSelection((prev) => {
              const newRowSelection = { ...prev };
              delete newRowSelection[sourceTarget.row.id];
              return newRowSelection;
            });
          }

          return;
        }

        if (dropTarget.tableId === tableId) {
          setSorting([]);

          if (sourceTarget.isSelected) {
            setRowSelection((prev) => ({ ...prev, [sourceTarget.row.id]: true }));
          }

          const edge = extractClosestEdge(location.current.dropTargets[0].data);

          const dropIndex = flatRows.findIndex((row) => row.id === dropTarget.row.id);
          const newData = [...flatRows];

          const insertIndex = edge === "bottom" ? dropIndex + 1 : dropIndex;
          newData.splice(insertIndex, 0, sourceTarget.row);

          setData(refreshOrders(newData));

          return;
        }

        return;
      },
    });
  }, [setData, setRowSelection, setSorting, table, tableId]);
};
