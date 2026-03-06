import { HTMLAttributes } from "react";

import { cn } from "@/utils";

interface ListLabelProps extends HTMLAttributes<HTMLSpanElement> {
  label: string;
}
export const ListLabel = ({ label, className, ...props }: ListLabelProps) => {
  return (
    <span className={cn("min-w-0 truncate font-medium", className)} {...props}>
      {label}
    </span>
  );
};
