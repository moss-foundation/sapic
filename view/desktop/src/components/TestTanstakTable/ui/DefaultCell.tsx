import { cn } from "@/utils";
import { Cell, flexRender } from "@tanstack/react-table";

function DefaultCell<TData>({ cell }: { cell: Cell<TData, unknown> }) {
  return (
    <td
      className={cn("border-1 border-l-0 border-[#E0E0E0] px-2 py-1.5")}
      style={{ width: cell.column.getSize() === 150 ? "auto" : cell.column.getSize() }}
      key={cell.id}
    >
      {flexRender(cell.column.columnDef.cell, cell.getContext())}
    </td>
  );
}

export { DefaultCell };
