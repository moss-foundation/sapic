import { HTMLAttributes, useEffect, useRef, useState } from "react";

import { DragHandleButton } from "@/components/DragHandleButton";
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

import { TestData } from "../types";
import { getTableRowData, isTableRow } from "../utils";

interface DefaultRowProps extends HTMLAttributes<HTMLTableRowElement> {
  disableDnd?: boolean;
  row: Row<TestData>;
  table: Table<TestData>;
  onAddNewRow?: () => void;
}

export const DefaultRow = ({
  row,
  children,
  className,
  disableDnd = false,
  table,
  onAddNewRow,
  ...props
}: DefaultRowProps) => {
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
            isSelected: row.getIsSelected(),
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
          if (!isTableRow(source)) return false;

          const sourceTableRowData = getTableRowData(source);

          if (
            sourceTableRowData.row.id === originalRow.id ||
            sourceTableRowData.tableType !== table.options.meta?.tableType
          ) {
            return false;
          }

          return true;
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
      <span className="peer/tableRow contents">{children}</span>

      {!disableDnd && (
        <DragHandleButton
          className={cn(
            "absolute top-1/2 -left-[8px] -translate-y-1/2 opacity-0 transition-opacity duration-100 peer-hover/tableRow:opacity-100 hover:opacity-100",
            {
              "opacity-100": isDragging,
            }
          )}
          ref={handleRef}
        />
      )}
      {closestEdge && <DropIndicator edge={closestEdge} gap={0} />}
      <AddNewRowDividerButton onClick={onAddNewRow} />
    </div>
  );
};

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
        <div className="background-(--moss-drag-handle-bg) absolute -top-[8px] -left-2 flex size-4 items-center justify-center rounded-sm p-px shadow">
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
