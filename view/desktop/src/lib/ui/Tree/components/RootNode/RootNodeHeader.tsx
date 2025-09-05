import { forwardRef, HTMLAttributes } from "react";

import { cn } from "@/utils";

import { ActiveNodeIndicator } from "../ActiveNodeIndicator";
import { useTreeContext } from "../TreeContext";

interface RootNodeHeaderProps extends HTMLAttributes<HTMLLIElement> {
  isActive?: boolean;
  children: React.ReactNode;
}

export const RootNodeHeader = forwardRef<HTMLLIElement, RootNodeHeaderProps>(
  ({ isActive = false, children, className, ...props }: RootNodeHeaderProps, ref) => {
    const { treePaddingLeft, treePaddingRight } = useTreeContext();

    return (
      <li
        ref={ref}
        className={cn(
          "group/TreeRootNodeHeader relative flex w-full min-w-0 items-center justify-between py-0.75",
          className
        )}
        style={{
          paddingLeft: treePaddingLeft,
          paddingRight: treePaddingRight,
        }}
        {...props}
      >
        <ActiveNodeIndicator isActive={isActive} />
        {children}
      </li>
    );
  }
);
