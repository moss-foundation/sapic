import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as MenuPrimitive from "@radix-ui/react-menu";

/* -------------------------------------------------------------------------------------------------
 * CheckboxItem
 * -----------------------------------------------------------------------------------------------*/

import { ScopedProps } from "./Menu";

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

export { CheckboxItem };

export type { CheckboxItemProps };
