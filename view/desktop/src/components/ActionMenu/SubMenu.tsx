import { forwardRef } from "react";

import { Menu } from "@/lib/ui";
import { cn } from "@/utils";

import { actionMenuContentStyles, actionMenuStyles } from "./styles";

const Sub = Menu.Sub;

const SubTrigger = forwardRef<HTMLDivElement, Menu.SubTriggerProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.SubTrigger ref={ref} className={cn(actionMenuStyles(), className)} {...props}>
      {children}
    </Menu.SubTrigger>
  );
});

const SubContent = forwardRef<HTMLDivElement, Menu.SubContentProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.SubContent ref={ref} className={cn(actionMenuContentStyles(), className)} {...props}>
      {children}
    </Menu.SubContent>
  );
});

export { Sub, SubContent, SubTrigger };
