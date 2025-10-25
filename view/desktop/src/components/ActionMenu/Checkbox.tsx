import { forwardRef } from "react";

import { Icon, Menu } from "@/lib/ui";
import { cn } from "@/utils";

import { actionMenuStyles } from "./styles";

const CheckboxItem = forwardRef<HTMLDivElement, Menu.CheckboxItemProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.CheckboxItem ref={ref} className={cn(actionMenuStyles(), className)} {...props}>
      {props.checked ? <Icon icon="GreenCheckmark" /> : <Icon icon="GreenCheckmark" className="opacity-0" />}

      <div className="flex w-full items-center gap-1.5">
        <span>{children}</span>

        {props.shortcut && <div className="ml-auto opacity-30">{props.shortcut}</div>}
      </div>
    </Menu.CheckboxItem>
  );
});

export { CheckboxItem };
