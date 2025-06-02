import { HTMLAttributes } from "react";

import { ActionMenu } from "@/components";
import Icon from "@/lib/ui/Icon";
import { cn } from "@/utils";
import { flexRender, Header } from "@tanstack/react-table";

interface DefaultHeaderProps<TData> extends HTMLAttributes<HTMLTableCellElement> {
  header: Header<TData, unknown>;
  tableHeight?: number;
}

export function DefaultHeader<TData>({ header, tableHeight, ...props }: DefaultHeaderProps<TData>) {
  const isLastColumn = header.column.getIsLastColumn();
  const isSortable = header.column.getCanSort();
  const canHide = header.column.getCanHide();

  const toggleSortingHandler = header.column.getToggleSortingHandler();

  return (
    <th
      className={cn("relative border-r border-b border-[#E0E0E0] px-2 py-1.5 capitalize", {
        "border-r-0": isLastColumn,
      })}
      style={{ width: header.getSize() }}
      {...props}
    >
      <div className="group flex items-center justify-center">
        <span
          className="relative cursor-pointer truncate text-center"
          onClick={isSortable ? toggleSortingHandler : undefined}
        >
          {header.isPlaceholder ? null : flexRender(header.column.columnDef.header, header.getContext())}
        </span>

        {isSortable && (
          <div
            //prettier-ignore
            className={cn(`
              h-full
              absolute top-0 right-0 
              flex items-center justify-center
              bg-[#F4F4F4] px-2 
              group-hover:opacity-100
              transition-opacity duration-100
           `)}
          >
            {header.column.getIsSorted() && (
              <button className="cursor-pointer" onClick={header.column.getToggleSortingHandler()}>
                {header.column.getIsSorted() === "asc" ? "🔼" : header.column.getIsSorted() === "desc" ? "🔽" : "🔼"}
              </button>
            )}

            <ActionMenu.Root>
              <ActionMenu.Trigger className="cursor-pointer rounded p-1 hover:bg-gray-300">
                <Icon icon="MoreVertical" />
              </ActionMenu.Trigger>

              <ActionMenu.Content>
                <ActionMenu.Item onClick={() => header.column.toggleSorting(false)}>
                  <button>asc</button>
                </ActionMenu.Item>
                <ActionMenu.Item onClick={() => header.column.toggleSorting(true)}>
                  <button>desc</button>
                </ActionMenu.Item>
                <ActionMenu.Item onClick={() => header.column.clearSorting()}>
                  <button>clear sorting</button>
                </ActionMenu.Item>
                {canHide && (
                  <ActionMenu.Item onClick={() => header.column.toggleVisibility()}>
                    <button>hide col</button>
                  </ActionMenu.Item>
                )}
              </ActionMenu.Content>
            </ActionMenu.Root>
          </div>
        )}
      </div>

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
