import React, { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as MenuPrimitive from "@radix-ui/react-menu";
import { useControllableState } from "@radix-ui/react-use-controllable-state";

import { Icon, type Icons } from "../Icon";
import { ScopedProps, useMenuScope } from "./Menu";
import { menuContentStyles, menuItemStyles } from "./styles";

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
        className={cn(menuItemStyles({ disabled: props.disabled }), props.className)}
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
    const { __scopeActionMenu, ...subContentProps } = props;

    return (
      <MenuPrimitive.SubContent
        {...subContentProps}
        ref={forwardedRef}
        style={{ ...props.style }}
        className={cn(menuContentStyles(), props.className)}
      />
    );
  }
);

export { Sub, SubContent, SubTrigger };

export type { ActionMenuSubProps, SubContentProps, SubTriggerProps };
