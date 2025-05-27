import { cn } from "@/utils";
import { flexRender, Header } from "@tanstack/react-table";

export function DefaultHeader<TData>({ header }: { header: Header<TData, unknown> }) {
  const resizeHandler = header.getResizeHandler();
  return (
    <th
      key={header.column.id}
      colSpan={header.colSpan}
      className={cn("relative p-2 text-left")}
      style={{
        width: header.column.getSize(),
      }}
    >
      <span className="max-w-full truncate">{flexRender(header.column.columnDef.header, header.getContext())}</span>
      <div
        onDoubleClick={() => header.column.resetSize()}
        onMouseDown={resizeHandler}
        className={`absolute top-0 right-0 h-full w-[5px] bg-blue-600 transition-colors duration-200 select-none ${
          header.column.getIsResizing() ? "bg-secondary" : ""
        }`}
        style={{ cursor: "col-resize", touchAction: "none" }}
      />
    </th>
  );
}

export default DefaultHeader;
