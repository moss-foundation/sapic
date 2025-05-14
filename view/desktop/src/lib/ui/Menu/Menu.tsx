import React, { ComponentPropsWithoutRef, ElementRef, forwardRef, useId } from "react";

import { cn } from "@/utils";
import { composeEventHandlers } from "@radix-ui/primitive";
import { createContextScope, Scope } from "@radix-ui/react-context";
import * as MenuPrimitive from "@radix-ui/react-menu";
import { createMenuScope } from "@radix-ui/react-menu";
import { Primitive } from "@radix-ui/react-primitive";
import { useCallbackRef } from "@radix-ui/react-use-callback-ref";

import { Icon, type Icons } from "../Icon";
import { composeRefs } from "./utils/compose-refs";

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

  triggerId: string;
  triggerRef: React.RefObject<HTMLButtonElement | null>;
  contentId: string;
  onOpenChange(open: boolean): void;
  onOpenToggle(): void;
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
  const triggerRef = React.useRef<HTMLButtonElement>(null);
  const handleOpenChange = React.useCallback(
    (open: boolean) => {
      setOpen(open);
      handleOpenChangeProp(open);
    },
    [handleOpenChangeProp]
  );

  return (
    <ActionMenuProvider
      triggerId={useId()}
      triggerRef={triggerRef}
      contentId={useId()}
      scope={__scopeActionMenu}
      open={open}
      onOpenChange={handleOpenChange}
      onOpenToggle={React.useCallback(() => setOpen((prevOpen) => !prevOpen), [setOpen])}
      modal={modal}
    >
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
    const { __scopeActionMenu, disabled = false, ...triggerProps } = props;
    const context = useActionMenuContext(TRIGGER_NAME, __scopeActionMenu);

    if (props.openOnRightClick) {
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
                    handleOpen(event);
                    event.preventDefault();
                  })
            }
            onPointerDown={
              disabled
                ? props.onPointerDown
                : composeEventHandlers(
                    props.onPointerDown,
                    whenTouchOrPen((event) => {
                      // clear the long press here in case there's multiple touch points
                      clearLongPress();
                      longPressTimerRef.current = window.setTimeout(() => handleOpen(event), 700);
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
    } else {
      const menuScope = useMenuScope(__scopeActionMenu);
      return (
        <MenuPrimitive.Anchor asChild {...menuScope}>
          <Primitive.button
            type="button"
            id={context.triggerId}
            aria-haspopup="menu"
            aria-expanded={context.open}
            aria-controls={context.open ? context.contentId : undefined}
            data-state={context.open ? "open" : "closed"}
            data-disabled={disabled ? "" : undefined}
            disabled={disabled}
            {...triggerProps}
            ref={composeRefs(forwardedRef, context.triggerRef)}
            onPointerDown={composeEventHandlers(props.onPointerDown, (event) => {
              // only call handler if it's the left button (mousedown gets triggered by all mouse buttons)
              // but not when the control key is pressed (avoiding MacOS right click)
              if (!disabled && event.button === 0 && event.ctrlKey === false) {
                context.onOpenToggle();
                // prevent trigger focusing when opening
                // this allows the content to be given focus without competition
                if (!context.open) event.preventDefault();
              }
            })}
            onKeyDown={composeEventHandlers(props.onKeyDown, (event) => {
              if (disabled) return;
              if (["Enter", " "].includes(event.key)) context.onOpenToggle();
              if (event.key === "ArrowDown") context.onOpenChange(true);
              // prevent keydown from scrolling window / first focused item to execute
              // that keydown (inadvertently closing the menu)
              if (["Enter", " ", "ArrowDown"].includes(event.key)) event.preventDefault();
            })}
          />
        </MenuPrimitive.Anchor>
      );
    }
  }
);

/* -------------------------------------------------------------------------------------------------
 * Content
 * -----------------------------------------------------------------------------------------------*/

type ContentElement = ElementRef<typeof MenuPrimitive.Content>;
type ContentProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.Content>>;

const Content = forwardRef<ContentElement, ContentProps>(({ className, align = "start", ...props }, forwardedRef) => {
  return (
    <MenuPrimitive.Content
      {...props}
      align={align}
      className={cn("z-50 rounded-lg px-1 py-1.5 shadow-lg", className)}
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
  alignWithIcons?: boolean;
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
      {props.alignWithIcons && <div className="size-4 shrink-0 opacity-0" />}

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

export default function mergeRefs<T>(...inputRefs: (React.Ref<T> | undefined)[]): React.Ref<T> | React.RefCallback<T> {
  const filteredInputRefs = inputRefs.filter(Boolean);

  if (filteredInputRefs.length <= 1) {
    const firstRef = filteredInputRefs[0];

    return firstRef || null;
  }

  return function mergedRefs(ref) {
    for (const inputRef of filteredInputRefs) {
      if (typeof inputRef === "function") {
        inputRef(ref);
      } else if (inputRef) {
        (inputRef as React.MutableRefObject<T | null>).current = ref;
      }
    }
  };
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
