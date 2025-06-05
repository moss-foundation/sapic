import { cn } from "@/utils";
import { Cell, flexRender } from "@tanstack/react-table";

function DefaultCell<TData>({ cell }: { cell: Cell<TData, unknown> }) {
  const isLastColumn = cell.column.getIsLastColumn();
  const isSelected = cell.row.getIsSelected();

  return (
    <div
      role="cell"
      className={cn("flex items-center justify-center border-r border-b border-(--moss-border-color)", {
        "border-r-0": isLastColumn,
        "opacity-60": !isSelected,
      })}
      style={{ width: cell.column.getSize() }}
      key={cell.id}
    >
      {flexRender(cell.column.columnDef.cell, cell.getContext())}
    </div>
  );
}

export { DefaultCell };
