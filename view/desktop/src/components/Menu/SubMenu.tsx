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
        className={cn("flex items-center gap-1.5 rounded px-2 py-1", {
          "cursor-not-allowed opacity-50": props.disabled,
          "hover:background-(--moss-menu-item-bg-hover) cursor-pointer hover:outline-hidden": !props.disabled,
        })}
      >
        {!hideIcon &&
          (props.icon ? (
            <Icon icon={props.icon} className="text-(--moss-menu-item-color)" />
          ) : (
            <Icon icon="DropdownMenuRadioIndicator" className="opacity-0" />
          ))}

        <span>{props.label}</span>

        <Icon icon="ArrowheadRight" className="ml-auto text-(--moss-menu-item-color)" />
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
        className={cn(
          "background-(--moss-menu-content-bg) rounded border border-(--moss-menu-content-border) px-3 py-2 text-(--moss-menu-content-text) shadow-lg",
          props.className
        )}
      />
    );
  }
);
