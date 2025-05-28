import { flexRender, Header } from "@tanstack/react-table";

export function DefaultHeader<TData>({ header }: { header: Header<TData, unknown> }) {
  return (
    <th
      className="relative border-r border-r-[#E0E0E0] px-2 py-1.5 capitalize"
      style={{ width: header.column.getSize() }}
    >
      <span className="relative cursor-pointer" onClick={header.column.getToggleSortingHandler()}>
        {header.isPlaceholder ? null : flexRender(header.column.columnDef.header, header.getContext())}
        {{
          asc: " 🔼",
          desc: " 🔽",
        }[header.column.getIsSorted() as string] ?? null}
      </span>

      {header.column.getCanResize() && (
        <div
          onClick={(e) => e.stopPropagation()}
          className="hover:background-(--moss-primary) absolute top-0 -right-[2px] h-full w-[4px] cursor-col-resize bg-transparent transition-colors duration-200 select-none"
          onMouseDown={header.getResizeHandler()}
        />
      )}
    </th>
  );
}

export default DefaultHeader;
