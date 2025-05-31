import { forwardRef, HTMLAttributes, useEffect, useRef, useState } from "react";

import DropIndicator from "@/components/DropIndicator";
import { cn } from "@/utils";
import {
  attachClosestEdge,
  extractClosestEdge,
  type Edge,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
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

  const originalRow = row.original;

  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  useEffect(() => {
    const handle = handleRef.current;
    const element = rowRef.current;

    if (!element || !handle || disableDnd) return;

    return combine(
      draggable({
        element,
        dragHandle: handle,
        getInitialData: () => ({
          type: "TableRow",
          data: {
            tableId: table.options.meta?.id,
            row: originalRow,
          },
        }),
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
          return source.data.type === "TableRow" && originalRow.key !== source.data.data.row.key;
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
    <>
      <tr ref={rowRef} className={cn("relative", className)} {...props}>
        {children}
        {!disableDnd && <RowHandle ref={handleRef} />}
        {closestEdge && <DropIndicator edge={closestEdge} gap={0} />}
      </tr>
    </>
  );
};

const RowHandle = forwardRef<HTMLDivElement>((_, ref) => {
  return (
    <div
      ref={ref}
      className="absolute top-1/2 -left-[8px] flex size-4 -translate-y-1/2 cursor-grab items-center justify-center rounded bg-white shadow"
    >
      <svg width="6" height="10" viewBox="0 0 6 10" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path
          fillRule="evenodd"
          clipRule="evenodd"
          d="M0 0H2V2H0V0ZM4 0H6V2H4V0ZM2 4H0V6H2V4ZM4 4H6V6H4V4ZM2 8H0V10H2V8ZM4 8H6V10H4V8Z"
          fill="#525252"
        />
      </svg>
    </div>
  );
});
