import { forwardRef, HTMLAttributes } from "react";

import { cn } from "@/utils";

import { NodeIndicator } from "../NodeIndicator";

interface RootNodeHeaderProps extends HTMLAttributes<HTMLLIElement> {
  isActive?: boolean;
  children: React.ReactNode;
  disableIndicator?: boolean;
  treePaddingLeft?: number;
  treePaddingRight?: number;
}

export const RootNodeHeader = forwardRef<HTMLLIElement, RootNodeHeaderProps>(
  (
    {
      isActive = false,
      children,
      className,
      disableIndicator = false,
      treePaddingLeft = 0,
      treePaddingRight = 0,
      ...props
    }: RootNodeHeaderProps,
    ref
  ) => {
    return (
      <li
        ref={ref}
        className={cn(
          "group/TreeRootNodeHeader py-0.75 relative flex w-full min-w-0 items-center justify-between",
          className
        )}
        style={{
          paddingLeft: treePaddingLeft,
          paddingRight: treePaddingRight,
        }}
        {...props}
      >
        {!disableIndicator && <NodeIndicator isActive={isActive} />}
        {children}
      </li>
    );
  }
);
