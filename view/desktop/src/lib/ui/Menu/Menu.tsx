import React, { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import { composeEventHandlers } from "@radix-ui/primitive";
import { createContextScope, Scope } from "@radix-ui/react-context";
import * as MenuPrimitive from "@radix-ui/react-menu";
import { createMenuScope } from "@radix-ui/react-menu";
import { Primitive } from "@radix-ui/react-primitive";
import { useCallbackRef } from "@radix-ui/react-use-callback-ref";

import { Icon, type Icons } from "../Icon";

type Direction = "ltr" | "rtl";
type Point = { x: number; y: number };

/* -------------------------------------------------------------------------------------------------
 * Root
 * -----------------------------------------------------------------------------------------------*/

const ACTION_MENU_NAME = "ActionMenu";

type ScopedProps<P> = P & { __scopeActionMenu?: Scope };
const [createActionMenuContext, createActionMenuScope] = createContextScope(ACTION_MENU_NAME, [createMenuScope]);
export const useMenuScope = createActionMenuScope();

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

function whenTouchOrPen<E>(handler: React.PointerEventHandler<E>): React.PointerEventHandler<E> {
  return (event) => (event.pointerType !== "mouse" ? handler(event) : undefined);
}

export { Content, Item, Portal, Root, Trigger };

export type {
  ActionMenuContextValue,
  ActionMenuProps,
  ActionMenuTriggerElement,
  ActionMenuTriggerProps,
  ContentElement,
  ContentProps,
  Direction,
  ItemElement,
  ItemProps,
  ScopedProps,
};
