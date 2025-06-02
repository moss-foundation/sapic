import { cn } from "@/utils";
import { Cell, flexRender } from "@tanstack/react-table";

function DefaultCell<TData>({ cell }: { cell: Cell<TData, unknown> }) {
  const isLastColumn = cell.column.getIsLastColumn();
  const isSelected = cell.row.getIsSelected();

  return (
    <td
      className={cn("border-r border-b border-[#E0E0E0] px-2 py-1.5", {
        "border-r-0": isLastColumn,
        "opacity-60": !isSelected,
      })}
      style={{ width: cell.column.getSize() }}
      key={cell.id}
    >
      {flexRender(cell.column.columnDef.cell, cell.getContext())}
    </td>
  );
}

export { DefaultCell };
