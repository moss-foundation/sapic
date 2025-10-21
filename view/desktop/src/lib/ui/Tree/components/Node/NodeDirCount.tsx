import { HTMLAttributes } from "react";

import { cn } from "@/utils";

interface NodeDirCountProps extends HTMLAttributes<HTMLDivElement> {
  count: number;
}

export const NodeDirCount = ({ count, className, ...props }: NodeDirCountProps) => {
  return (
    <div className={cn("text-(--moss-list-descriptionForeground)", className)} {...props}>
      ({count})
    </div>
  );
};
