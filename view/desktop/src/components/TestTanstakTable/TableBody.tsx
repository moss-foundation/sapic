import { HTMLAttributes, TdHTMLAttributes } from "react";

import { cn } from "@/utils";

function Body({ children, ...props }: HTMLAttributes<HTMLTableSectionElement>) {
  return <tbody {...props}>{children}</tbody>;
}

function Row({ children, ...props }: HTMLAttributes<HTMLTableRowElement>) {
  return <tr {...props}>{children}</tr>;
}

function Cell({ children, className, ...props }: TdHTMLAttributes<HTMLTableCellElement>) {
  return (
    <td className={cn("border-1 border-[#E0E0E0] px-2 py-1.5", className)} {...props}>
      {children}
    </td>
  );
}

export { Body, Cell, Row };
