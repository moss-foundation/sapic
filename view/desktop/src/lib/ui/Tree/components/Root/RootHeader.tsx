import { forwardRef, HTMLAttributes } from "react";

import { cn } from "@/utils";

import { ActivityIndicator } from "../ActivityIndicator";

interface RootHeaderProps extends HTMLAttributes<HTMLLIElement> {
  isActive?: boolean;
  children: React.ReactNode;
  disableIndicator?: boolean;
  paddingLeft?: number;
  paddingRight?: number;
}

export const RootHeader = forwardRef<HTMLLIElement, RootHeaderProps>(
  (
    {
      isActive = false,
      children,
      className,
      disableIndicator = false,
      paddingLeft = 0,
      paddingRight = 0,
      ...props
    }: RootHeaderProps,
    ref
  ) => {
    return (
      <li
        ref={ref}
        className={cn(
          "group/TreeRootHeader py-0.75 relative flex w-full min-w-0 items-center justify-between",
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
