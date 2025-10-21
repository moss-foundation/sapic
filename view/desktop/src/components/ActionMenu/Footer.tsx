import { forwardRef } from "react";

import { Menu } from "@/lib/ui";
import { cn } from "@/utils";

const Footer = forwardRef<HTMLDivElement, Menu.LabelProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.Footer ref={ref} className={cn("background-(--moss-secondary-background) text-left", className)} {...props}>
      {children}
    </Menu.Footer>
  );
});

export { Footer };
