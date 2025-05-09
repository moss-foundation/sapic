import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as MenuPrimitive from "@radix-ui/react-menu";

import Icon, { Icons } from "../Icon";
import { ScopedProps } from "./types";

/* -------------------------------------------------------------------------------------------------
 * SubTrigger
 * -----------------------------------------------------------------------------------------------*/

export type SubTriggerElement = ElementRef<typeof MenuPrimitive.SubTrigger>;
export type SubTriggerProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.SubTrigger>> & {
  label: string;
  icon?: Icons;
  hideIcon?: boolean;
};

export const SubTrigger = forwardRef<SubTriggerElement, SubTriggerProps>(
  ({ hideIcon = false, ...props }, forwardedRef) => {
    return (
      <MenuPrimitive.SubTrigger
        {...props}
        ref={forwardedRef}
        className={cn(
          "flex items-center gap-1.5 rounded px-2 py-1",
          {
            "cursor-not-allowed opacity-50": props.disabled,
            "cursor-pointer hover:outline-hidden": !props.disabled,
          },
          props.className
        )}
      >
        {!hideIcon &&
          (props.icon ? (
            <Icon icon={props.icon} className="opacity-40" />
          ) : (
            <Icon icon="RadioIndicator" className="opacity-0" />
          ))}

        <span>{props.label}</span>

        <Icon icon="ChevronRight" className="ml-auto opacity-40" />
      </MenuPrimitive.SubTrigger>
    );
  }
);

/* -------------------------------------------------------------------------------------------------
 * SubContent
 * -----------------------------------------------------------------------------------------------*/

export type SubContentElement = ElementRef<typeof MenuPrimitive.Content>;
export type SubContentProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.SubContent>>;

export const SubContent = forwardRef<SubContentElement, SubContentProps>(
  (props: ScopedProps<SubContentProps>, forwardedRef) => {
    return (
      <MenuPrimitive.SubContent
        {...props}
        ref={forwardedRef}
        sideOffset={16}
        style={{ ...props.style }}
        className={cn("z-50 min-w-36 rounded-lg px-1 py-1.5 shadow-lg", props.className)}
      />
    );
  }
);
