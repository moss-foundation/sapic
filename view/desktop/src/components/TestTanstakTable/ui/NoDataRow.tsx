import { cn } from "@/utils";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { dropTargetForElements, monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { useEffect, useRef, useState } from "react";
import { DnDRowData, TestData } from "../DataTable";

interface NoDataRowProps {
  colSpan: number;
  setData: (data: TestData[]) => void;
  tableId: string;
}

export const NoDataRow = ({ colSpan, setData, tableId }: NoDataRowProps) => {
  const ref = useRef<HTMLTableRowElement>(null);
  const [isDraggedOver, setIsDraggedOver] = useState(false);

  useEffect(() => {
    const element = ref.current;
    if (!element) return;

    return combine(
      monitorForElements({
        canMonitor: ({ source }) => {
          return source.data.type === "TableRow" || source.data.type === "TableRowNoResults";
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
      <td colSpan={colSpan} className={cn("h-24 text-center", isDraggedOver && "background-(--moss-info-background)")}>
        No results.
      </td>
    </tr>
  );
};
