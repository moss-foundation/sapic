import { HTMLAttributes } from "react";

import { cn } from "@/utils";

interface NodeLabelProps extends HTMLAttributes<HTMLSpanElement> {
  label: string;
  isRootNode?: boolean;
}
export const NodeLabel = ({ label, className, isRootNode, ...props }: NodeLabelProps) => {
  return (
    <span
      className={cn(
        "min-w-0 truncate",
        {
          "capitalize": isRootNode,
        },
        className
      )}
      {...props}
    >
      {label}
    </span>
  );
};
