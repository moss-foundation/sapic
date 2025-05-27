import { Cell, flexRender } from "@tanstack/react-table";

function DefaultCell<TData>({ cell }: { cell: Cell<TData, unknown> }) {
  return (
    <td
      key={cell.id}
      style={{
        width: cell.column.getSize(),
      }}
    >
      <span className="block max-w-full truncate">{flexRender(cell.column.columnDef.cell, cell.getContext())}</span>
    </td>
  );
}

export { DefaultCell };
