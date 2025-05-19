import { forwardRef } from "react";

import { Menu } from "@/lib/ui";
import { cn } from "@/utils";

const SectionLabel = forwardRef<HTMLDivElement, Menu.LabelProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.SectionLabel ref={ref} className={cn("text-(--moss-not-selected-item-color)", className)} {...props}>
      {children}
    </Menu.SectionLabel>
  );
});

export { SectionLabel };
