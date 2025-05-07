import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as MenuPrimitive from "@radix-ui/react-menu";

import Icon from "../Icon";
import { ScopedProps } from "./types";

export type CheckboxItemElement = ElementRef<typeof MenuPrimitive.CheckboxItem>;
export type CheckboxItemProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.CheckboxItem>> & {
  label: string;
  shortcut?: string[];
  disabled?: boolean;
};

export const CheckboxItem = forwardRef<CheckboxItemElement, CheckboxItemProps>(
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
        {props.checked ? <Icon icon="GreenCheckmark" /> : <Icon icon="GreenCheckmark" className="opacity-0" />}

        <div className="flex w-full items-center gap-2.5">
          <span>{props.label}</span>

          {props.shortcut && <div className="ml-auto opacity-30">{props.shortcut.join("")}</div>}
        </div>
      </MenuPrimitive.CheckboxItem>
    );
  }
);
