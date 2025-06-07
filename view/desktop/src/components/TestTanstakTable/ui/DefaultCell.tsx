import { cn } from "@/utils";
import { Cell, flexRender } from "@tanstack/react-table";

import { TestData } from "../types";

function DefaultCell({ cell, focusOnMount }: { cell: Cell<TestData, unknown>; focusOnMount?: boolean }) {
  const isLastColumn = cell.column.getIsLastColumn();

  return (
    <div
      key={cell.id}
      role="cell"
      className={cn("flex items-center justify-center border-r border-b border-(--moss-border-color)", {
        "border-r-0": isLastColumn,
      })}
      style={{ width: cell.column.getSize() }}
    >
      {flexRender(cell.column.columnDef.cell, { ...cell.getContext(), focusOnMount })}
    </div>
  );
}

export { DefaultCell };
