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
  onAddNewRow?: () => void;
}

export const DefaultRow = <TData,>({
  row,
  children,
  className,
  disableDnd = false,
  table,
  onAddNewRow,
  ...props
}: DefaultRowProps<TData>) => {
  const handleRef = useRef<HTMLDivElement>(null);
  const rowRef = useRef<HTMLTableRowElement>(null);

  const originalRow = row.original;

  const [closestEdge, setClosestEdge] = useState<Edge | null>(null);

  const [isDragging, setIsDragging] = useState(false);

  useEffect(() => {
    const dragHandle = handleRef.current;
    const element = rowRef.current;

    if (!element || !dragHandle || disableDnd) return;

    return combine(
      draggable({
        element,
        dragHandle,
        getInitialData: () => ({
          type: "TableRow",
          data: {
            tableId: table.options.meta?.tableId,
            tableType: table.options.meta?.tableType,
            row: originalRow,
          },
        }),
        onDragStart() {
          setIsDragging(true);
        },
        onDrop() {
          setIsDragging(false);
        },
      }),
      dropTargetForElements({
        element,
        getIsSticky() {
          return true;
        },
        getData({ input }) {
          return attachClosestEdge(
            {
              type: "TableRow",
              data: {
                tableId: table.options.meta?.tableId,
                tableType: table.options.meta?.tableType,
                row: originalRow,
              },
            },
            {
              element,
              input,
              allowedEdges: ["top", "bottom"],
            }
          );
        },
        canDrop({ source }) {
          return source.data.type === "TableRow" && originalRow.id !== source.data.data.row.id;
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
    <div
      role="row"
      ref={rowRef}
      className={cn(
        "relative flex",
        {
          "background-(--moss-table-cell-bg) brightness-90": isDragging,
        },
        className
      )}
      {...props}
    >
      <span className="peer/tableRow flex">{children}</span>

      {!disableDnd && (
        <RowHandle
          className="opacity-0 transition-opacity duration-100 peer-hover/tableRow:opacity-100"
          ref={handleRef}
          isDragging={isDragging}
        />
      )}
      {closestEdge && <DropIndicator edge={closestEdge} gap={0} />}
      <AddNewRowDividerButton onClick={onAddNewRow} />
    </div>
  );
};

const RowHandle = forwardRef<HTMLDivElement, { isDragging: boolean; className: string }>(
  ({ isDragging, className }, ref) => {
    return (
      <div
        ref={ref}
        className={cn(
          "background-(--moss-table-handle-bg) absolute top-1/2 -left-[8px] flex size-4 -translate-y-1/2 cursor-grab items-center justify-center rounded shadow",
          isDragging && "opacity-100!",
          className
        )}
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
  }
);

const AddNewRowDividerButton = ({ onClick }: { onClick?: () => void }) => {
  const [visible, setVisible] = useState(false);
  const timeoutIdRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const handleMouseEnter = () => {
    timeoutIdRef.current = setTimeout(() => {
      setVisible(true);
      timeoutIdRef.current = null;
    }, 600);
  };

  const handleMouseLeave = () => {
    if (timeoutIdRef.current) {
      clearTimeout(timeoutIdRef.current);
      timeoutIdRef.current = null;
    }
    setVisible(false);
  };

  const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.stopPropagation();
    onClick?.();
  };

  return (
    <button
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      //prettier-ignore
      className={cn(`
          absolute -top-[1px] left-0 z-100
          w-full h-[2px]
          
          background-(--moss-primary)
          cursor-pointer

          transition-opacity duration-100
   
          before:h-[5px] before:w-full before:content-[''] before:absolute before:left-0 before:-top-[5px]
          after:h-[5px] after:w-full after:content-[''] after:absolute after:left-0 after:-bottom-[5px]
         `,
        {
          "opacity-0": !visible,
        }
      )}
      onClick={visible ? handleClick : undefined}
    >
      <div className="relative h-full w-full">
        <div className="background-(--moss-table-handle-bg) absolute -top-[8px] -left-2 flex size-4 items-center justify-center rounded-sm p-px shadow">
          <DividerButtonIcon />
        </div>
      </div>
    </button>
  );
};

const DividerButtonIcon = () => {
  return (
    <svg width="8" height="8" viewBox="0 0 8 8" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M4.5 3.5V0H3.5V3.5H0V4.5H3.5V8H4.5V4.5H8V3.5H4.5Z" fill="#525252" />
    </svg>
  );
};
