import React, { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import { composeEventHandlers } from "@radix-ui/primitive";
import { createContext, createContextScope, Scope } from "@radix-ui/react-context";
import * as MenuPrimitive from "@radix-ui/react-menu";
import { createMenuScope } from "@radix-ui/react-menu";
import { Primitive } from "@radix-ui/react-primitive";
import { useCallbackRef } from "@radix-ui/react-use-callback-ref";
import { useControllableState } from "@radix-ui/react-use-controllable-state";

import { Icon, type Icons } from "../Icon";

type Direction = "ltr" | "rtl";
type Point = { x: number; y: number };

/* -------------------------------------------------------------------------------------------------
 * Root
 * -----------------------------------------------------------------------------------------------*/

const ACTION_MENU_NAME = "ActionMenu";

type ScopedProps<P> = P & { __scopeActionMenu?: Scope };
const [createActionMenuContext, createActionMenuScope] = createContextScope(ACTION_MENU_NAME, [createMenuScope]);
const useMenuScope = createActionMenuScope();

type ActionMenuContextValue = {
  open: boolean;
  onOpenChange(open: boolean): void;
  modal: boolean;
};

const [ActionMenuProvider, useActionMenuContext] = createActionMenuContext<ActionMenuContextValue>(ACTION_MENU_NAME);

interface ActionMenuProps {
  children?: React.ReactNode;
  onOpenChange?(open: boolean): void;
  dir?: Direction;
  modal?: boolean;
}

const Root: React.FC<ActionMenuProps> = (props: ScopedProps<ActionMenuProps>) => {
  const { __scopeActionMenu, children, onOpenChange, dir, modal = true } = props;
  const [open, setOpen] = React.useState(false);
  const menuScope = useMenuScope(__scopeActionMenu);
  const handleOpenChangeProp = useCallbackRef(onOpenChange);

  const handleOpenChange = React.useCallback(
    (open: boolean) => {
      setOpen(open);
      handleOpenChangeProp(open);
    },
    [handleOpenChangeProp]
  );

  return (
    <ActionMenuProvider scope={__scopeActionMenu} open={open} onOpenChange={handleOpenChange} modal={modal}>
      <MenuPrimitive.Root {...menuScope} dir={dir} open={open} onOpenChange={handleOpenChange} modal={modal}>
        {children}
      </MenuPrimitive.Root>
    </ActionMenuProvider>
  );
};

/* -------------------------------------------------------------------------------------------------
 * Trigger
 * -----------------------------------------------------------------------------------------------*/

const TRIGGER_NAME = "ActionMenuTrigger";

type ActionMenuTriggerElement = React.ComponentRef<typeof Primitive.span>;
type PrimitiveSpanProps = React.ComponentPropsWithoutRef<typeof Primitive.span>;
interface ActionMenuTriggerProps extends PrimitiveSpanProps {
  disabled?: boolean;
  openOnRightClick?: boolean;
}

const Trigger = React.forwardRef<ActionMenuTriggerElement, ActionMenuTriggerProps>(
  (props: ScopedProps<ActionMenuTriggerProps>, forwardedRef) => {
    const { __scopeActionMenu, disabled = false, openOnRightClick = false, className, ...triggerProps } = props;
    const context = useActionMenuContext(TRIGGER_NAME, __scopeActionMenu);
    const menuScope = useMenuScope(__scopeActionMenu);
    const pointRef = React.useRef<Point>({ x: 0, y: 0 });
    const virtualRef = React.useRef({
      getBoundingClientRect: () => DOMRect.fromRect({ width: 0, height: 0, ...pointRef.current }),
    });
    const longPressTimerRef = React.useRef(0);
    const clearLongPress = React.useCallback(() => window.clearTimeout(longPressTimerRef.current), []);
    const handleOpen = (event: React.MouseEvent | React.PointerEvent) => {
      pointRef.current = { x: event.clientX, y: event.clientY };
      context.onOpenChange(true);
    };

    React.useEffect(() => clearLongPress, [clearLongPress]);
    React.useEffect(() => void (disabled && clearLongPress()), [disabled, clearLongPress]);

    return (
      <>
        <MenuPrimitive.Anchor {...menuScope} virtualRef={virtualRef} />
        <Primitive.span
          data-state={context.open ? "open" : "closed"}
          data-disabled={disabled ? "" : undefined}
          {...triggerProps}
          ref={forwardedRef}
          className={cn(
            {
              "cursor-pointer": !disabled || openOnRightClick,
            },
            className
          )}
          // prevent iOS context menu from appearing
          style={{ WebkitTouchCallout: "none", ...props.style }}
          // if trigger is disabled, enable the native Context Menu
          onContextMenu={
            disabled
              ? props.onContextMenu
              : composeEventHandlers(props.onContextMenu, (event) => {
                  // clearing the long press here because some platforms already support
                  // long press to trigger a `contextmenu` event
                  clearLongPress();
                  if (openOnRightClick && event.button === 2) {
                    handleOpen(event);
                    event.preventDefault();
                  }
                })
          }
          onPointerDown={
            disabled
              ? (event) => {
                  event?.preventDefault();
                }
              : composeEventHandlers(
                  (event) => {
                    if (!openOnRightClick && event.button === 0) {
                      handleOpen(event);
                    }
                    props.onPointerDown?.(event);
                  },
                  whenTouchOrPen((event) => {
                    // clear the long press here in case there's multiple touch points
                    if (openOnRightClick && event.button === 2) {
                      clearLongPress();
                      longPressTimerRef.current = window.setTimeout(() => handleOpen(event), 700);
                    }
                  })
                )
          }
          onPointerMove={
            disabled ? props.onPointerMove : composeEventHandlers(props.onPointerMove, whenTouchOrPen(clearLongPress))
          }
          onPointerCancel={
            disabled
              ? props.onPointerCancel
              : composeEventHandlers(props.onPointerCancel, whenTouchOrPen(clearLongPress))
          }
          onPointerUp={
            disabled ? props.onPointerUp : composeEventHandlers(props.onPointerUp, whenTouchOrPen(clearLongPress))
          }
        />
      </>
    );
  }
);

/* -------------------------------------------------------------------------------------------------
 * Content
 * -----------------------------------------------------------------------------------------------*/

type ContentElement = ElementRef<typeof MenuPrimitive.Content>;
type ContentProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.Content>>;

const Content = forwardRef<ContentElement, ContentProps>((props, forwardedRef) => {
  return (
    <MenuPrimitive.Content
      {...props}
      className={cn("z-50 rounded-lg px-1 py-1.5 shadow-lg", props.className)}
      ref={forwardedRef}
    />
  );
});

const Portal = MenuPrimitive.Portal;

const Label = MenuPrimitive.Label;

/* -------------------------------------------------------------------------------------------------
 * Item
 * -----------------------------------------------------------------------------------------------*/

type ItemElement = ElementRef<typeof MenuPrimitive.Item>;
type ItemProps = {
  shortcut?: string;
  disabled?: boolean;
  icon?: Icons;
  leftIconPadding?: boolean;
  iconClassName?: string;
} & React.ComponentPropsWithoutRef<typeof MenuPrimitive.Item>;

const Item = forwardRef<ItemElement, ItemProps>(({ iconClassName, className, ...props }, forwardedRef) => {
  return (
    <MenuPrimitive.Item
      {...props}
      ref={forwardedRef}
      className={cn(
        "flex items-center gap-1.5 rounded py-0.5 pr-5 pl-[7px]",
        {
          "cursor-not-allowed grayscale-100": props.disabled,
          "cursor-pointer hover:outline-hidden": !props.disabled,
        },
        className
      )}
    >
      {props.icon && <Icon icon={props.icon} className={cn("shrink-0", iconClassName)} />}
      {props.leftIconPadding && <div className="size-4 shrink-0 opacity-0" />}

      <div className="flex w-full items-center gap-2.5">
        <span>{props.children}</span>

        {props.shortcut && <div className="ml-auto opacity-30">{props.shortcut}</div>}
      </div>
    </MenuPrimitive.Item>
  );
});

/* -------------------------------------------------------------------------------------------------
 * CheckboxItem
 * -----------------------------------------------------------------------------------------------*/

type CheckboxItemElement = ElementRef<typeof MenuPrimitive.CheckboxItem>;
type CheckboxItemProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.CheckboxItem>> & {
  shortcut?: string;
  disabled?: boolean;
};

const CheckboxItem = forwardRef<CheckboxItemElement, CheckboxItemProps>(
  (props: ScopedProps<CheckboxItemProps>, forwardedRef) => {
    return (
      <MenuPrimitive.CheckboxItem
        {...props}
        ref={forwardedRef}
        className={cn(
          "flex items-center gap-1.5 rounded py-0.5 pr-5 pl-[7px]",
          {
            "cursor-not-allowed opacity-50": props.disabled,
            "cursor-pointer hover:outline-hidden": !props.disabled,
          },
          props.className
        )}
      >
        {props.children}
      </MenuPrimitive.CheckboxItem>
    );
  }
);

/* -------------------------------------------------------------------------------------------------
 * RadioGroup
 * -----------------------------------------------------------------------------------------------*/

type RadioGroupElement = ElementRef<typeof MenuPrimitive.RadioGroup>;
type RadioGroupProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.RadioGroup>>;

const RadioGroup = forwardRef<RadioGroupElement, RadioGroupProps>(
  (props: ScopedProps<RadioGroupProps>, forwardedRef) => {
    return <MenuPrimitive.RadioGroup {...props} ref={forwardedRef} />;
  }
);

/* -------------------------------------------------------------------------------------------------
 * RadioItem
 * -----------------------------------------------------------------------------------------------*/

type RadioItemElement = ElementRef<typeof MenuPrimitive.RadioItem>;
type RadioItemProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.RadioItem>> & {
  checked: boolean;
  disabled?: boolean;
};

const RadioItem = forwardRef<RadioItemElement, RadioItemProps>((props: ScopedProps<RadioItemProps>, forwardedRef) => {
  return (
    <MenuPrimitive.RadioItem
      {...props}
      ref={forwardedRef}
      className={cn(
        "flex items-center gap-1.5 rounded py-0.5 pr-5 pl-[7px]",
        {
          "cursor-not-allowed opacity-50": props.disabled,
          "cursor-pointer hover:outline-hidden": !props.disabled,
        },
        props.className
      )}
    >
      {props.checked ? (
        <Icon icon="MenuRadioIndicator" className="size-4" />
      ) : (
        <Icon icon="MenuRadioIndicator" className="size-4 opacity-0" />
      )}

      <div className="flex w-full items-center gap-2.5">
        <span>{props.children}</span>
      </div>
    </MenuPrimitive.RadioItem>
  );
});

const Separator = ({ className, ...props }: ComponentPropsWithoutRef<"div">) => {
  return <div className={cn("my-1 h-px w-full", className)} {...props} />;
};

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
  hideIcon?: boolean;
};

const SubTrigger = forwardRef<SubTriggerElement, SubTriggerProps>(({ hideIcon = false, ...props }, forwardedRef) => {
  const { __scopeActionMenu, ...triggerItemProps } = props;

  return (
    <MenuPrimitive.SubTrigger
      {...triggerItemProps}
      ref={forwardedRef}
      className={cn(
        "flex items-center gap-1.5 rounded py-0.5 pr-5 pl-[7px]",
        {
          "cursor-not-allowed opacity-50": props.disabled,
          "cursor-pointer hover:outline-hidden": !props.disabled,
        },
        props.className
      )}
    >
      {!hideIcon &&
        (props.icon ? <Icon icon={props.icon} className="" /> : <Icon icon="RadioIndicator" className="opacity-0" />)}

      <span>{props.children}</span>

      <Icon icon="ChevronRight" className="ml-auto" />
    </MenuPrimitive.SubTrigger>
  );
});

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
        sideOffset={16}
        style={{ ...props.style }}
        className={cn("z-50 min-w-36 rounded-lg px-1 py-1.5 shadow-lg", props.className)}
      />
    );
  }
);

function whenTouchOrPen<E>(handler: React.PointerEventHandler<E>): React.PointerEventHandler<E> {
  return (event) => (event.pointerType !== "mouse" ? handler(event) : undefined);
}

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
        className={cn("flex cursor-pointer items-center gap-1.5 rounded py-0.5 pr-5 pl-[7px]", className)}
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
        <Icon icon={context.open ? "ChevronDown" : "ChevronRight"} className="opacity-40" />
        {children}
        {props.total && <span className="ml-1 text-(--moss-not-selected-item-color)">{props.total}</span>}
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

export {
  Accordion,
  AccordionContent,
  AccordionTrigger,
  CheckboxItem,
  Content,
  Item,
  Label,
  Portal,
  RadioGroup,
  RadioItem,
  Root,
  Separator,
  Sub,
  SubContent,
  SubTrigger,
  Trigger,
};

export type {
  AccordionContextValue,
  AccordionProps,
  AccordionTriggerElement,
  AccordionTriggerProps,
  ActionMenuContextValue,
  ActionMenuProps,
  ActionMenuSubProps,
  ActionMenuTriggerElement,
  ActionMenuTriggerProps,
  CheckboxItemProps,
  ContentElement,
  ContentProps,
  Direction,
  ItemElement,
  ItemProps,
  RadioGroupElement,
  RadioGroupProps,
  RadioItemElement,
  RadioItemProps,
  ScopedProps,
  SubContentElement,
  SubContentProps,
  SubTriggerElement,
  SubTriggerProps,
};
