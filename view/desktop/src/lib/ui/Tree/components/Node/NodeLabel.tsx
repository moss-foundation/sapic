import { HTMLAttributes } from "react";

import { cn } from "@/utils";

interface NodeLabelProps extends HTMLAttributes<HTMLSpanElement> {
  label: string;
  capitalize?: boolean;
}
export const NodeLabel = ({ label, className, capitalize, ...props }: NodeLabelProps) => {
  return (
    <span
      className={cn(
        "min-w-0 truncate",
        {
          "capitalize": capitalize,
        },
        className
      )}
      {...props}
    >
      {label}
    </span>
  );
};
