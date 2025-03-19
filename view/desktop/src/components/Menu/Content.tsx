import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as MenuPrimitive from "@radix-ui/react-menu";

import { ScopedProps } from "./types";

export type ContentElement = ElementRef<typeof MenuPrimitive.Content>;
export type ContentProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.Content>>;

export const Content = forwardRef<ContentElement, ContentProps>((props, forwardedRef) => {
  return (
    <MenuPrimitive.Content
      {...props}
      className={cn(
        "background-(--moss-menu-content-bg) z-50 rounded-lg border border-(--moss-menu-content-border) px-3 py-1.5 text-(--moss-menu-content-text) shadow-lg",
        props.className
      )}
      ref={forwardedRef}
    />
  );
});
