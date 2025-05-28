import { flexRender, Header } from "@tanstack/react-table";

export function DefaultHeader<TData>({ header }: { header: Header<TData, unknown> }) {
  return (
    <th className="relative bg-[#F5F5F5] capitalize" style={{ width: header.column.getSize() }}>
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
          className="absolute top-0 -right-[3px] h-full w-[6px] cursor-col-resize bg-blue-600 transition-colors duration-200 select-none"
          onMouseDown={header.getResizeHandler()}
        />
      )}
    </th>
  );
}

export default DefaultHeader;
