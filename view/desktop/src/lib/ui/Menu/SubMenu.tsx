import React, { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as MenuPrimitive from "@radix-ui/react-menu";
import { useControllableState } from "@radix-ui/react-use-controllable-state";

import { Icon, type Icons } from "../Icon";
import { ScopedProps, useMenuScope } from "./Menu";

/* -------------------------------------------------------------------------------------------------
 * Sub
 * -----------------------------------------------------------------------------------------------*/

const SUB_NAME = "ActionMenuSub";

interface ActionMenuSubProps {
  children?: React.ReactNode;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?(open: boolean): void;
}

const Sub: React.FC<ActionMenuSubProps> = (props: ScopedProps<ActionMenuSubProps>) => {
  const { __scopeActionMenu, children, onOpenChange, open: openProp, defaultOpen = false } = props;
  const menuScope = useMenuScope(__scopeActionMenu);

  const [open, setOpen] = useControllableState({
    prop: openProp,
    defaultProp: defaultOpen,
    onChange: onOpenChange,
    caller: SUB_NAME,
  });

  return (
    <MenuPrimitive.Sub {...menuScope} open={open} onOpenChange={setOpen}>
      {children}
    </MenuPrimitive.Sub>
  );
};

/* -------------------------------------------------------------------------------------------------
 * SubTrigger
 * -----------------------------------------------------------------------------------------------*/

type SubTriggerElement = ElementRef<typeof MenuPrimitive.SubTrigger>;
type SubTriggerProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.SubTrigger>> & {
  icon?: Icons;
  iconClassName?: string;
  alignWithIcons?: boolean;
};

const SubTrigger = forwardRef<SubTriggerElement, SubTriggerProps>(
  ({ alignWithIcons = false, iconClassName, ...props }, forwardedRef) => {
    const { __scopeActionMenu, ...triggerItemProps } = props;

    return (
      <MenuPrimitive.SubTrigger
        {...triggerItemProps}
        ref={forwardedRef}
        className={cn(
          "flex items-center gap-1.5 rounded py-0.5 pr-2 pl-[7px]",
          {
            "cursor-not-allowed opacity-50": props.disabled,
            "cursor-pointer hover:outline-hidden": !props.disabled,
          },
          props.className
        )}
      >
        {props.icon && <Icon icon={props.icon} className={cn("shrink-0", iconClassName)} />}
        {alignWithIcons && <div className={cn("size-4 shrink-0 opacity-0", iconClassName)} />}

        <span className="truncate">{props.children}</span>

        <Icon icon="ChevronRight" className="ml-auto" />
      </MenuPrimitive.SubTrigger>
    );
  }
);

/* -------------------------------------------------------------------------------------------------
 * SubContent
 * -----------------------------------------------------------------------------------------------*/

type SubContentElement = ElementRef<typeof MenuPrimitive.Content>;
type SubContentProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.SubContent>>;

const SubContent = forwardRef<SubContentElement, SubContentProps>(
  (props: ScopedProps<SubContentProps>, forwardedRef) => {
    return (
      <MenuPrimitive.SubContent
        {...props}
        ref={forwardedRef}
        style={{ ...props.style }}
        className={cn("z-50 min-w-36 rounded-lg px-1 py-1.5 shadow-lg", props.className)}
      />
    );
  }
);

export { Sub, SubContent, SubTrigger };

export type { ActionMenuSubProps, SubContentProps, SubTriggerProps };
