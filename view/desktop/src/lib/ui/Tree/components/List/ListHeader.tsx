import { HTMLAttributes, ReactNode, RefObject } from "react";

import { cn } from "@/utils";

interface ListHeaderProps extends HTMLAttributes<HTMLHeadingElement> {
  ref?: RefObject<HTMLHeadingElement | null>;
  children: ReactNode;
  paddingLeft?: number;
  paddingRight?: number;
}

export const ListHeader = ({
  ref,
  children,
  className,
  paddingLeft = 0,
  paddingRight = 0,
  ...props
}: ListHeaderProps) => {
  return (
    <h3
      ref={ref}
      className={cn(
        "group/TreeListActions relative flex w-full min-w-0 items-center justify-between py-[6px]",
        className
      )}
      style={{ paddingLeft, paddingRight }}
      {...props}
    >
      {children}
    </h3>
  );
};
