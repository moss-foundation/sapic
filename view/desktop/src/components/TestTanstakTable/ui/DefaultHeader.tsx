import { HTMLAttributes } from "react";

import { cn } from "@/utils";
import { flexRender, Header } from "@tanstack/react-table";

interface DefaultHeaderProps<TData> extends HTMLAttributes<HTMLTableCellElement> {
  header: Header<TData, unknown>;
  tableHeight?: number;
}

export function DefaultHeader<TData>({ header, tableHeight, ...props }: DefaultHeaderProps<TData>) {
  return (
    <th
      className="relative border-r border-r-[#E0E0E0] px-2 py-1.5 capitalize"
      style={{ width: header.getSize() }}
      {...props}
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
          className={cn(
            "hover:background-(--moss-primary) absolute top-0 -right-[2px] z-10 w-[4px] cursor-col-resize bg-transparent transition-colors duration-200 select-none",
            {
              "background-(--moss-primary)": header.column.getIsResizing(),
            }
          )}
          onMouseDown={header.getResizeHandler()}
          style={{ height: tableHeight ? tableHeight - 1 : "100%" }}
        />
      )}
    </th>
  );
}

export default DefaultHeader;
