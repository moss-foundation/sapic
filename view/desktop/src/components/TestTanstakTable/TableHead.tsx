import { HTMLAttributes, ThHTMLAttributes } from "react";

import { Icon } from "@/lib/ui";
import { HeaderContext } from "@tanstack/react-table";

function Head({ children, ...props }: React.HTMLAttributes<HTMLTableSectionElement>) {
  return <thead {...props}>{children}</thead>;
}

function Row({ children, ...props }: HTMLAttributes<HTMLTableRowElement>) {
  return (
    <tr className="bg-[#F4F4F4]" {...props}>
      {children}
    </tr>
  );
}

interface HeadCellProps<T, K> extends Omit<ThHTMLAttributes<HTMLTableCellElement>, "children"> {
  info: HeaderContext<T, K>;
  name: string;
}

function Cell<T, K>({ info, name, ...props }: HeadCellProps<T, K>) {
  const { table } = info;
  const sorted = info.column.getIsSorted();

  return (
    <th
      className="flex cursor-pointer items-center justify-center gap-1 bg-green-400 px-2 py-1.5 text-left"
      onClick={(e) => {
        e.preventDefault();
        info.column.toggleSorting(info.column.getIsSorted() === "asc");
        if (info.column.getIsSorted() === "desc") {
          info.column.clearSorting();
        }
      }}
      {...props}
    >
      {name}
      {!sorted && <Icon icon="ChevronUp" className="opacity-0" />}
      {sorted === "asc" && <Icon icon="ChevronUp" />}
      {sorted === "desc" && <Icon icon="ChevronDown" />}
    </th>
  );
}

export { Cell, Head, Row };
