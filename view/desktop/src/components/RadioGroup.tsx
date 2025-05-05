import { ComponentPropsWithoutRef, createContext, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as RadioGroupPrimitive from "@radix-ui/react-radio-group";

export interface RadioRootProps {
  className?: string;
}

const RadioGroupContext = createContext<RadioRootProps>({});

const Root = forwardRef<
  ElementRef<typeof RadioGroupPrimitive.Root>,
  ComponentPropsWithoutRef<typeof RadioGroupPrimitive.Root> & RadioRootProps
>(({ className, ...props }, forwardedRef) => {
  return (
    <RadioGroupContext.Provider value={{}}>
      <RadioGroupPrimitive.Root {...props} ref={forwardedRef} className={className} />
    </RadioGroupContext.Provider>
  );
});

export interface RadioItemProps {
  className?: string;
}

const defaultRadioGroupItemStyles = `
  flex justify-center items-center cursor-pointer rounded-full size-[18px] group
  background-(--moss-radio-bg) border-1 border-solid border-(--moss-radio-border) 
  
  hover:brightness-95
  
  disabled:hover:brightness-100
  disabled:background-(--moss-radio-bg-disabled)
  disabled:opacity-50
  disabled:border-(--moss-radio-border-disabled)
  disabled:data-[state=checked]:background-(--moss-radio-bg-disabled)
  disabled:cursor-default
  
  data-[state=checked]:background-(--moss-primary) 
  data-[state=checked]:border-none

  focus-visible:outline-2 
  focus-visible:outline-(--moss-primary) 
  focus-visible:outline-offset-2
`;

const Item = forwardRef<
  ElementRef<typeof RadioGroupPrimitive.Item>,
  ComponentPropsWithoutRef<typeof RadioGroupPrimitive.Item> & RadioItemProps
>((props, forwardedRef) => {
  return (
    <RadioGroupPrimitive.Item
      {...props}
      ref={forwardedRef}
      className={cn(defaultRadioGroupItemStyles, props.className)}
    />
  );
});
const Indicator = forwardRef<
  ElementRef<typeof RadioGroupPrimitive.Indicator>,
  ComponentPropsWithoutRef<typeof RadioGroupPrimitive.Indicator> & {
    className?: string;
  }
>((props, forwardedRef) => {
  return <RadioGroupPrimitive.Indicator {...props} ref={forwardedRef} className={props.className} />;
});

export { Indicator, Item, Root };
