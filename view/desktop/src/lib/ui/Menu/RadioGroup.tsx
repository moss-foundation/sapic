import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as MenuPrimitive from "@radix-ui/react-menu";

import { Icon } from "../Icon";
import { ScopedProps } from "./Menu";
import { menuIconStyles } from "./styles";

/* -------------------------------------------------------------------------------------------------
 * RadioGroup
 * -----------------------------------------------------------------------------------------------*/

type RadioGroupElement = ElementRef<typeof MenuPrimitive.RadioGroup>;
type RadioGroupProps = ScopedProps<ComponentPropsWithoutRef<typeof MenuPrimitive.RadioGroup>>;

const RadioGroup = forwardRef<RadioGroupElement, RadioGroupProps>(
  (props: ScopedProps<RadioGroupProps>, forwardedRef) => {
    const { __scopeActionMenu, ...radioGroupProps } = props;

    return <MenuPrimitive.RadioGroup {...radioGroupProps} ref={forwardedRef} />;
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
  const { __scopeActionMenu, ...radioItemProps } = props;

  return (
    <MenuPrimitive.RadioItem
      {...radioItemProps}
      ref={forwardedRef}
      className={cn(
        "flex items-center gap-1.5 rounded py-0.5 pl-[7px] pr-5",
        {
          "cursor-not-allowed opacity-50": props.disabled,
          "hover:outline-hidden cursor-pointer": !props.disabled,
        },
        props.className
      )}
    >
      {props.checked ? (
        <Icon icon="MenuRadioIndicator" className={menuIconStyles()} />
      ) : (
        <Icon icon="MenuRadioIndicator" className={menuIconStyles({ className: "opacity-0" })} />
      )}

      <div className="flex w-full items-center gap-1.5">
        <span>{props.children}</span>
      </div>
    </MenuPrimitive.RadioItem>
  );
});

export { RadioGroup, RadioItem };

export type { RadioGroupProps, RadioItemProps };
