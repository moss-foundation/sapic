import { HTMLAttributes, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import DropIndicator from "@/components/DropIndicator";
import { cn } from "@/utils";
import {
  attachClosestEdge,
  extractClosestEdge,
  type Edge,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";
import { Row, Table } from "@tanstack/react-table";

interface DefaultRowProps<TData> extends HTMLAttributes<HTMLTableRowElement> {
  disableDnd?: boolean;
  row: Row<TData>;
  table: Table<TData>;
}

export const DefaultRow = <TData,>({
  row,
  children,
  className,
  disableDnd = false,
  table,
  ...props
}: DefaultRowProps<TData>) => {
  const handleRef = useRef<HTMLDivElement>(null);
  const rowRef = useRef<HTMLTableRowElement>(null);

  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  const originalRow = row.original;

  useEffect(() => {
    const handle = handleRef.current;
    const element = rowRef.current;

    if (!element || !handle || disableDnd) return;

    return combine(
      draggable({
        element: handle,
        getInitialData: () => ({
          type: "TableRow",
          data: {
            tableId: table.options.meta?.id,
            row: originalRow,
          },
        }),
        onDrop: () => {
          setPreview(null);
        },
        onGenerateDragPreview({ nativeSetDragImage }) {
          setCustomNativeDragPreview({
            nativeSetDragImage,
            render({ container }) {
              setPreview((prev) => (prev === container ? prev : container));
            },
          });
        },
      }),
      dropTargetForElements({
        element,
        getIsSticky() {
          return true;
        },
        getData({ input }) {
          return attachClosestEdge(
            { type: "TableRow", data: { tableId: table.options.meta?.id, row: originalRow } },
            {
              element,
              input,
              allowedEdges: ["top", "bottom"],
            }
          );
        },
        canDrop({ source }) {
          return source.data.type === "TableRow";
        },
        onDrop() {
          setClosestEdge(null);
        },
        onDragLeave() {
          setClosestEdge(null);
        },
        onDragEnter({ self }) {
          const closestEdge = extractClosestEdge(self.data);
          setClosestEdge(closestEdge);
        },
        onDrag({ self }) {
          const closestEdge = extractClosestEdge(self.data);

          setClosestEdge((current) => {
            if (current === closestEdge) return current;

            return closestEdge;
          });
        },
      })
    );
  });

  return (
    <tr ref={rowRef} className={cn("relative", className)} {...props}>
      {children}
      {!disableDnd && (
        <div ref={handleRef} className="absolute top-1/2 -left-[8px] size-4 -translate-y-1/2 bg-red-500" />
      )}
      {closestEdge && <DropIndicator edge={closestEdge} gap={0} />}
      {preview && createPortal(<DefaultRow table={table} row={row} />, preview)}
    </tr>
  );
};
