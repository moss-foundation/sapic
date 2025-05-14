import { Menu } from "@/lib/ui";

const Accordion = Menu.Accordion;
const AccordionTrigger = ({ children, ...props }: Menu.AccordionTriggerProps) => {
  return <Menu.AccordionTrigger {...props}>{children}</Menu.AccordionTrigger>;
};
const AccordionContent = Menu.AccordionContent;

export { Accordion, AccordionContent, AccordionTrigger };
