import { forwardRef } from "react";

import { Icon, Menu } from "@/lib/ui";
import { cn } from "@/utils";

const CheckboxItem = forwardRef<HTMLDivElement, Menu.CheckboxItemProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.CheckboxItem
      ref={ref}
      className={cn("hover:background-(--moss-secondary-background-hover)", className)}
      {...props}
    >
      {props.checked ? <Icon icon="GreenCheckmark" /> : <Icon icon="GreenCheckmark" className="opacity-0" />}

      <div className="flex w-full items-center gap-2.5">
        <span>{children}</span>

        {props.shortcut && <div className="ml-auto opacity-30">{props.shortcut}</div>}
      </div>
    </Menu.CheckboxItem>
  );
});

export { CheckboxItem };
