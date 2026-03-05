import { HTMLAttributes, ReactNode, RefObject } from "react";

import { cn } from "@/utils";

interface ListHeaderProps extends HTMLAttributes<HTMLHeadingElement> {
  ref?: RefObject<HTMLHeadingElement | null>;
  children: ReactNode;
  offsetLeft?: number;
}

export const ListHeader = ({ ref, children, className, offsetLeft = 0, ...props }: ListHeaderProps) => {
  return (
    <h3
      ref={ref}
      className={cn("py-0.75 relative flex w-full min-w-0 items-center justify-between", className)}
      style={{ paddingLeft: offsetLeft }}
      {...props}
    >
      {children}
    </h3>
  );
};
