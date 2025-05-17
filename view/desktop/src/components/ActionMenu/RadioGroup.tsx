import { forwardRef } from "react";

import { Menu } from "@/lib/ui";
import { cn } from "@/utils";

import { actionMenuStyles } from "./styles";

const RadioGroup = Menu.RadioGroup;
const RadioItem = forwardRef<HTMLDivElement, Menu.RadioItemProps>(({ className, ...props }, ref) => {
  return <Menu.RadioItem ref={ref} className={cn(actionMenuStyles(), className)} {...props} />;
});

export { RadioGroup, RadioItem };
