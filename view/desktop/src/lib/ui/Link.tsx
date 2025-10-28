import React, { forwardRef } from "react";

import { cn } from "@/utils";

export interface LinkProps extends React.AnchorHTMLAttributes<HTMLAnchorElement> {
  children: React.ReactNode;
  className?: string;
}

export const Link = forwardRef<HTMLAnchorElement, LinkProps>(({ className, children, ...props }, ref) => {
  return (
    <a
      ref={ref}
      className={cn(
        "text-(--moss-link-foreground) hover:text-(--moss-link-foreground-hover) cursor-pointer underline-offset-4 transition-colors",
        className
      )}
      {...props}
    >
      {children}
    </a>
  );
});

export default Link;
