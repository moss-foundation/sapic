import { HTMLAttributes, ReactNode, RefObject } from "react";

import { cn } from "@/utils";

interface ListHeaderProps extends HTMLAttributes<HTMLHeadingElement> {
  ref?: RefObject<HTMLHeadingElement | null>;
  children: ReactNode;
  offsetLeft?: number;
  offsetRight?: number;
}

export const ListHeader = ({
  ref,
  children,
  className,
  offsetLeft = 0,
  offsetRight = 0,
  ...props
}: ListHeaderProps) => {
  return (
    <h3
      ref={ref}
      className={cn(
        "group/TreeListActions relative flex w-full min-w-0 items-center justify-between py-[6px]",
        className
      )}
      style={{ paddingLeft: offsetLeft, paddingRight: offsetRight }}
      {...props}
    >
      {children}
    </h3>
  );
};
