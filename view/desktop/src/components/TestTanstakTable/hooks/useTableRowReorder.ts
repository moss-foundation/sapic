import { swapListByIndexWithEdge } from "@/utils/swapListByIndexWithEdge";
import { extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { SortingState, Table } from "@tanstack/react-table";
import { Dispatch, SetStateAction, useEffect } from "react";

interface TestData {
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

interface DnDRowData {
  type: "TableRow";
  data: {
    tableId: string;
    row: TestData;
  };
}

interface TableRowReorderProps {
  table: Table<TestData>;
  tableId: string;
  setSorting: (sorting: SortingState) => void;
  setData: Dispatch<SetStateAction<TestData[]>>;
}

export const useTableRowReorder = ({ table, tableId, setSorting, setData }: TableRowReorderProps) => {
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
            const sourceIndex = flatRows.findIndex((row) => row.id === sourceTarget.row.id);
            const dropIndex = flatRows.findIndex((row) => row.id === dropTarget.row.id);

            const newData = swapListByIndexWithEdge(sourceIndex, dropIndex, flatRows, edge);
            setData(newData);
          }

          return;
        }

        if (sourceTarget.tableId === tableId) {
          setSorting([]);

          setData((prev) => {
            return [...prev].filter((row) => row.id !== sourceTarget.row.id);
          });

          return;
        }

        if (dropTarget.tableId === tableId) {
          setSorting([]);
          const edge = extractClosestEdge(location.current.dropTargets[0].data);

          const dropIndex = flatRows.findIndex((row) => row.id === dropTarget.row.id);
          const newData = [...flatRows];

          const insertIndex = edge === "bottom" ? dropIndex + 1 : dropIndex;
          newData.splice(insertIndex, 0, sourceTarget.row);

          setData(newData);

          return;
        }

        return;
      },
    });
  }, [table, tableId]);
};
