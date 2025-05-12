import { forwardRef } from "react";

import { Menu } from "@/lib/ui";
import { cn } from "@/utils";

const RadioGroup = Menu.RadioGroup;
const RadioItem = forwardRef<HTMLDivElement, Menu.RadioItemProps>(({ className, ...props }, ref) => {
  return (
    <Menu.RadioItem
      ref={ref}
      className={cn("hover:background-(--moss-secondary-background-hover)", className)}
      {...props}
    />
  );
});

export { RadioGroup, RadioItem };
