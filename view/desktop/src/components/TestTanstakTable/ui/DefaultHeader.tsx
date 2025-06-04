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
  const [showActionMenuButton, setShowActionMenuButton] = useState(false);

  const toggleSortingHandler = header.column.getToggleSortingHandler();

  return (
    <th
      className={cn(
        "group/tableHeader relative border-r border-b border-(--moss-border-color) px-2 py-1.5 capitalize",
        {
          "border-r-0": isLastColumn,
        }
      )}
      style={{ width: header.getSize() }}
      onMouseEnter={() => setShowActionMenuButton(true)}
      onMouseLeave={() => {
        setShowActionMenuButton(false);
      }}
      {...props}
    >
      <div
        className={cn("group relative flex items-center gap-2", {
          "justify-center": isFirstColumn,
          "justify-between": !isFirstColumn,
        })}
      >
        <span className="relative cursor-pointer truncate" onClick={isSortable ? toggleSortingHandler : undefined}>
          {header.isPlaceholder ? null : flexRender(header.column.columnDef.header, header.getContext())}
        </span>

        {isSortable && (
          <div className="background-(--moss-table-header-bg) absolute top-0 right-0 flex h-full items-center justify-center gap-1">
            {header.column.getIsSorted() && (
              <button className="cursor-pointer" onClick={header.column.getToggleSortingHandler()}>
                {header.column.getIsSorted() === "asc" ? "🔼" : header.column.getIsSorted() === "desc" ? "🔽" : "🔼"}
              </button>
            )}

            <ActionMenu.Root onOpenChange={setShowActionMenuButton}>
              <ActionMenu.Trigger
                className={cn(
                  "background-(--moss-icon-secondary-background-hover) -my-px cursor-pointer rounded p-0.5 transition-opacity duration-100",
                  {
                    "opacity-100": showActionMenuButton,
                    "sr-only opacity-0": !showActionMenuButton,
                  }
                )}
                onClick={() => {
                  setShowActionMenuButton(true);
                }}
              >
                <Icon icon="MoreVertical" />
              </ActionMenu.Trigger>

              <ActionMenu.Content
                onMouseLeave={(e) => {
                  e.stopPropagation();
                }}
              >
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
          onClick={(e) => e.stopPropagation()}
          onMouseDown={header.getResizeHandler()}
          style={{ height: tableHeight ? tableHeight - 1 : "100%" }}
        />
      )}
    </th>
  );
}

export default DefaultHeader;
