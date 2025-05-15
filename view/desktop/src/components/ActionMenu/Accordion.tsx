import { Menu } from "@/lib/ui";
import { cn } from "@/utils";

import { actionMenuStyles } from "./styles";

const Accordion = Menu.Accordion;
const AccordionTrigger = ({ children, className, ...props }: Menu.AccordionTriggerProps) => {
  return (
    <Menu.AccordionTrigger className={cn(actionMenuStyles(), className)} {...props}>
      {children}
    </Menu.AccordionTrigger>
  );
};
const AccordionContent = Menu.AccordionContent;

export { Accordion, AccordionContent, AccordionTrigger };
