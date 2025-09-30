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
        "cursor-pointer text-[var(--moss-link-text)] underline-offset-4 transition-colors hover:text-[var(--moss-link-hover)]",
        className
      )}
      {...props}
    >
      {children}
    </a>
  );
});

Link.displayName = "Link";

export default Link;
