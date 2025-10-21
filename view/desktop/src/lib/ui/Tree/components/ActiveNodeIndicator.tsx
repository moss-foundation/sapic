import { HTMLAttributes } from "react";

import { cn } from "@/utils";

interface ActiveNodeIndicatorProps extends HTMLAttributes<HTMLDivElement> {
  isActive: boolean;
}

export const ActiveNodeIndicator = ({ isActive, className, ...props }: ActiveNodeIndicatorProps) => {
  return (
    <div
      //prettier-ignore
      className={cn(`
          absolute top-0 left-0 
          h-full w-full 
          group-hover/TreeRootNodeHeader:background-(--moss-secondary-background-hover)
          group-hover/TreeNodeControls:background-(--moss-secondary-background-hover)
          -z-1
        `,
        {
          "background-(--moss-secondary-background-hover) border-l border-l-(--moss-accent)": isActive,
        },
        className 
      )}
      {...props}
    />
  );
};
