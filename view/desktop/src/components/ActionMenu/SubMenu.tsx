import { forwardRef } from "react";

import { Menu } from "@/lib/ui";
import { cn } from "@/utils";

const Sub = Menu.Sub;

const SubTrigger = forwardRef<HTMLDivElement, Menu.SubTriggerProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.SubTrigger
      ref={ref}
      className={cn("hover:background-(--moss-secondary-background-hover)", className)}
      {...props}
    >
      {children}
    </Menu.SubTrigger>
  );
});

const SubContent = forwardRef<HTMLDivElement, Menu.SubContentProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.SubContent
      ref={ref}
      className={cn("background-(--moss-primary-background) border border-(--moss-border-color)", className)}
      {...props}
    >
      {children}
    </Menu.SubContent>
  );
});

export { Sub, SubContent, SubTrigger };
