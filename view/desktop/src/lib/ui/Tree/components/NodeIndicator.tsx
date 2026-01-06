import { HTMLAttributes } from "react";

import { cn } from "@/utils";

interface NodeIndicatorProps extends HTMLAttributes<HTMLDivElement> {
  isActive: boolean;
  isDirty: boolean;
}

export const NodeIndicator = ({ isActive, isDirty, className, ...props }: NodeIndicatorProps) => {
  return (
    <div
      //prettier-ignore
      className={cn(`
          absolute top-0 left-0 
          h-full w-full 
          -z-1
        `,
        {
          "background-(--moss-secondary-background-hover) border-l border-l-(--moss-accent)": isActive && !isDirty,
          "bg-orange-200": isDirty,
          "group-hover/TreeRootNodeHeader:background-(--moss-secondary-background-hover) group-hover/TreeNodeControls:background-(--moss-secondary-background-hover)": !isDirty,
        },
        className 
      )}
      {...props}
    />
  );
};
