import React, { ComponentPropsWithoutRef, ElementRef, forwardRef, useId } from "react";

import { cn } from "@/utils";
import { composeEventHandlers } from "@radix-ui/primitive";
import { createContextScope, Scope } from "@radix-ui/react-context";
import * as MenuPrimitive from "@radix-ui/react-menu";
import { createMenuScope } from "@radix-ui/react-menu";
import { Primitive } from "@radix-ui/react-primitive";
import { useControllableState } from "@radix-ui/react-use-controllable-state";

import { Icon, type Icons } from "../Icon";
import { menuContentStyles, menuIconStyles, menuItemStyles } from "./styles";
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
  open?: boolean;
}

const Root: React.FC<ActionMenuProps> = (props: ScopedProps<ActionMenuProps>) => {
  const { __scopeActionMenu, children, onOpenChange, dir, modal = true, open: openProp } = props;
  const menuScope = useMenuScope(__scopeActionMenu);
  const triggerRef = React.useRef<HTMLButtonElement>(null);
  const [open, setOpen] = useControllableState({
    prop: openProp,
    defaultProp: false,
    onChange: onOpenChange,
    caller: ACTION_MENU_NAME,
  });

  return (
    <ActionMenuProvider
      scope={__scopeActionMenu}
      triggerId={useId()}
      triggerRef={triggerRef}
      contentId={useId()}
      open={open}
      onOpenChange={setOpen}
      onOpenToggle={React.useCallback(() => setOpen((prevOpen) => !prevOpen), [setOpen])}
      modal={modal}
    >
      <MenuPrimitive.Root {...menuScope} dir={dir} open={open} onOpenChange={setOpen} modal={modal}>
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
    const { __scopeActionMenu, disabled = false, openOnRightClick = false, ...triggerProps } = props;
    const context = useActionMenuContext(TRIGGER_NAME, __scopeActionMenu);

    //Hooks only for right click
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

    if (openOnRightClick) {
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
      return (
        <MenuPrimitive.Anchor asChild>
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

const Content = forwardRef<ContentElement, ContentProps>(
  ({ className, align = "start", sideOffset = 4, ...props }, forwardedRef) => {
    const { __scopeActionMenu, ...contentProps } = props;

    return (
      <MenuPrimitive.Content
        {...contentProps}
        align={align}
        sideOffset={sideOffset}
        className={cn(menuContentStyles(), className)}
        ref={forwardedRef}
      />
    );
  }
);

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
  shortcutClassName?: string;
} & React.ComponentPropsWithoutRef<typeof MenuPrimitive.Item>;

const Item = forwardRef<ItemElement, ItemProps>(
  ({ iconClassName, alignWithIcons = false, icon, shortcut, shortcutClassName, className, ...props }, forwardedRef) => {
    return (
      <MenuPrimitive.Item
        {...props}
        ref={forwardedRef}
        className={cn(menuItemStyles({ disabled: props.disabled }), className)}
      >
        {icon && <Icon icon={icon} className={menuIconStyles({ className: iconClassName })} />}
        {alignWithIcons && !icon && <div className={menuIconStyles({ className: "opacity-0" })} />}

        <div className="flex w-full min-w-0 flex-1 items-center justify-between gap-1.5">
          <span className="w-full min-w-0 truncate">{props.children}</span>
          {shortcut && <span className={cn("shrink-0", shortcutClassName)}>{shortcut}</span>}
        </div>
      </MenuPrimitive.Item>
    );
  }
);

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
