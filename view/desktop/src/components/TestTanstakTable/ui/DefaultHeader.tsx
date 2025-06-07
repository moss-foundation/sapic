import { HTMLAttributes, useState } from "react";

import { ActionMenu } from "@/components";
import Icon from "@/lib/ui/Icon";
import { cn } from "@/utils";
import { flexRender, Header } from "@tanstack/react-table";

interface DefaultHeaderProps<TData> extends HTMLAttributes<HTMLTableCellElement> {
  header: Header<TData, unknown>;
  tableHeight?: number;
}

export function DefaultHeader<TData>({ header, tableHeight, ...props }: DefaultHeaderProps<TData>) {
  const isFirstColumn = header.column.getIsFirstColumn();
  const isLastColumn = header.column.getIsLastColumn();
  const isSortable = header.column.getCanSort();
  const canHide = header.column.getCanHide();

  const [showActionMenu, setShowActionMenu] = useState(false);

  const toggleSortingHandler = header.column.getToggleSortingHandler();

  if (isFirstColumn) {
    return (
      <div
        role="columnheader"
        className="flex items-center justify-center border-r border-b border-(--moss-border-color) py-1.5"
        style={{ width: header.getSize() }}
      >
        {flexRender(header.column.columnDef.header, header.getContext())}
      </div>
    );
  }

  return (
    <div
      role="columnheader"
      className={cn("relative border-r border-b border-(--moss-border-color) px-2 py-1.5", {
        "border-r-0": isLastColumn,
      })}
      style={{ width: header.getSize() }}
      {...props}
    >
      <div className={cn("group/tableHeader relative flex items-center gap-2")}>
        <button
          className={cn("relative cursor-pointer truncate capitalize", {
            "cursor-default": isLastColumn,
          })}
          onClick={isSortable ? toggleSortingHandler : undefined}
        >
          {header.isPlaceholder ? null : flexRender(header.column.columnDef.header, header.getContext())}
        </button>

        {isSortable && (
          <div className="background-(--moss-table-header-bg) absolute top-0 right-0 flex h-full items-center justify-center gap-1">
            {header.column.getIsSorted() && (
              <button className="cursor-pointer" onClick={toggleSortingHandler}>
                {header.column.getIsSorted() === "asc" ? "🔼" : header.column.getIsSorted() === "desc" ? "🔽" : "🔼"}
              </button>
            )}

            <ActionMenu.Root open={showActionMenu} onOpenChange={setShowActionMenu}>
              <ActionMenu.Trigger
                className={cn(
                  "background-(--moss-icon-secondary-background-hover) sr-only -my-px cursor-pointer rounded p-0.5! opacity-0 transition-opacity duration-100 group-hover/tableHeader:not-sr-only group-hover/tableHeader:opacity-100",
                  {
                    "not-sr-only opacity-100": showActionMenu,
                  }
                )}
              >
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
          className={cn(
            "hover:background-(--moss-primary) absolute top-0 -right-0.5 z-10 w-1 cursor-col-resize bg-transparent transition-colors duration-200 select-none",
            {
              "background-(--moss-primary)": header.column.getIsResizing(),
            }
          )}
          onMouseDown={header.getResizeHandler()}
          style={{ height: tableHeight ? tableHeight - 1 : "100%" }}
        />
      )}
    </div>
  );
}

export default DefaultHeader;
