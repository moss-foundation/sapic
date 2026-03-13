import { HTMLAttributes } from "react";

import { cn } from "@/utils";

interface NodeIndicatorProps extends HTMLAttributes<HTMLDivElement> {
  isActive: boolean;
  isDirty?: boolean;
}

export const ActivityIndicator = ({ isActive, isDirty = false, className, ...props }: NodeIndicatorProps) => {
  return (
    <div
      className={cn(
        "absolute left-0 top-0",
        "h-full w-full",
        "-z-2",
        {
          "background-(--moss-secondary-background-hover) border-l-(--moss-accent) border-l": isActive && !isDirty,
          "bg-orange-200": isDirty,
          "group-hover/TreeRootHeader:background-(--moss-secondary-background-hover) group-hover/TreeNodeDetails:background-(--moss-secondary-background-hover)":
            !isDirty,
        },
        className
      )}
      {...props}
    />
  );
};
