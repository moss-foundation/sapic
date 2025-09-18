import React, { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import { createContext } from "@radix-ui/react-context";
import { useControllableState } from "@radix-ui/react-use-controllable-state";

import { Icon } from "../Icon";
import { ScopedProps } from "./Menu";

/* -------------------------------------------------------------------------------------------------
 * Accordion
 * -----------------------------------------------------------------------------------------------*/

const ACCORDION_NAME = "ActionMenuAccordion";

type AccordionContextValue = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
};

const [AccordionProvider, useAccordionContext] = createContext<AccordionContextValue>(ACCORDION_NAME);

interface AccordionProps {
  children?: React.ReactNode;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
}

const Accordion: React.FC<AccordionProps> = (props: AccordionProps) => {
  const { children, defaultOpen = false, onOpenChange } = props;
  const [open, setOpen] = useControllableState({
    prop: undefined,
    defaultProp: defaultOpen,
    onChange: onOpenChange,
  });

  return (
    <AccordionProvider open={open} onOpenChange={setOpen}>
      {children}
    </AccordionProvider>
  );
};

/* -------------------------------------------------------------------------------------------------
 * AccordionTrigger
 * -----------------------------------------------------------------------------------------------*/

type AccordionTriggerElement = ElementRef<"div">;
type AccordionTriggerProps = ComponentPropsWithoutRef<"div"> & {
  total?: number;
};

const AccordionTrigger = forwardRef<AccordionTriggerElement, AccordionTriggerProps>(
  (props: ScopedProps<AccordionTriggerProps>, forwardedRef) => {
    const { __scopeActionMenu, className, children, ...triggerProps } = props;
    const context = useAccordionContext(ACCORDION_NAME);

    return (
      <div
        role="button"
        tabIndex={0}
        aria-expanded={context.open}
        {...triggerProps}
        ref={forwardedRef}
        className={cn("flex cursor-pointer items-center gap-2.5 rounded py-0.5 pr-3 pl-4", className)}
        onClick={(e) => {
          e.stopPropagation();
          context.onOpenChange(!context.open);
          triggerProps.onClick?.(e);
        }}
        onKeyDown={(e) => {
          if (e.key === "Enter" || e.key === " ") {
            e.preventDefault();
            context.onOpenChange(!context.open);
          }
        }}
      >
        <Icon icon={context.open ? "ChevronDown" : "ChevronRight"} />
        {children}
        {props.total !== undefined && <span className="text-(--moss-secondary-text)">{props.total}</span>}
      </div>
    );
  }
);

/* -------------------------------------------------------------------------------------------------
 * AccordionContent
 * -----------------------------------------------------------------------------------------------*/

type AccordionContentElement = ElementRef<"div">;
type AccordionContentProps = ComponentPropsWithoutRef<"div">;

const AccordionContent = forwardRef<AccordionContentElement, AccordionContentProps>(
  (props: ScopedProps<AccordionContentProps>, forwardedRef) => {
    const { __scopeActionMenu, className, children, ...contentProps } = props;
    const context = useAccordionContext(ACCORDION_NAME);

    if (!context.open) return null;

    return (
      <div {...contentProps} ref={forwardedRef} className={cn("pl-4", className)}>
        {children}
      </div>
    );
  }
);

export { Accordion, AccordionContent, AccordionTrigger };

export type { AccordionContentProps, AccordionProps, AccordionTriggerProps };
