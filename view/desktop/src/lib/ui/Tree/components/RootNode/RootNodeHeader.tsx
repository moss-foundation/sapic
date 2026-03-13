import { forwardRef, HTMLAttributes } from "react";

import { cn } from "@/utils";

import { ActivityIndicator } from "../ActivityIndicator";

interface RootNodeHeaderProps extends HTMLAttributes<HTMLLIElement> {
  isActive?: boolean;
  children: React.ReactNode;
  disableIndicator?: boolean;
  paddingLeft?: number;
  paddingRight?: number;
}

export const RootNodeHeader = forwardRef<HTMLLIElement, RootNodeHeaderProps>(
  (
    {
      isActive = false,
      children,
      className,
      disableIndicator = false,
      paddingLeft = 0,
      paddingRight = 0,
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
          paddingLeft,
          paddingRight,
        }}
        {...props}
      >
        {!disableIndicator && <ActivityIndicator isActive={isActive} />}
        {children}
      </li>
    );
  }
);
