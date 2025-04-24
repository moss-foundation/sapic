import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as CheckboxPrimitive from "@radix-ui/react-checkbox";

export interface CheckboxProps {
  className?: string;
}

const defaultCheckboxRootStyles = `border-1 border-solid border-(--moss-checkbox-border) rounded flex justify-center items-center size-4 text-white
  focus-visible:outline-2
  focus-visible:outline-background-(--moss-primary)
  focus-visible:outline-offset-2
  focus-visible:outline

  hover:brightness-95

  data-[state=checked]:border-none
  data-[state=checked]:background-(--moss-primary)
  data-[state=indeterminate]:background-(--moss-primary)
  data-[state=indeterminate]:border-none

  disabled:border-(--moss-checkbox-border-disabled)!
  disabled:background-(--moss-checkbox-bg-disabled)!

  disabled:pointer-events-none
  disabled:hover:brightness-100
  disabled:shadow-none
  disabled:cursor-not-allowed
`;

const CheckboxRoot = forwardRef<
  ElementRef<typeof CheckboxPrimitive.Root>,
  ComponentPropsWithoutRef<typeof CheckboxPrimitive.Root> & CheckboxProps
>(({ className, ...props }: CheckboxProps, forwardedRef) => {
  return <CheckboxPrimitive.Root ref={forwardedRef} className={cn(defaultCheckboxRootStyles, className)} {...props} />;
});

const Root = CheckboxRoot;
const Indicator = CheckboxPrimitive.Indicator;

export { Indicator, Root };
