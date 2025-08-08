import { cva } from "class-variance-authority";
import React from "react";

import { cn } from "@/utils";

import Icon, { Icons } from "./Icon";

export interface InputProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, "size"> {
  iconLeft?: Icons;
  iconRight?: Icons;
  iconClassName?: string;
}

//prettier-ignore
const inputStyles = cva(`
    peer 
    flex items-center w-full gap-2
    rounded-sm py-0 font-medium transition-[outline] 
    has-[input:focus-within]:outline 
    has-[input:focus-within]:-outline-offset-1`,
  {
    variants: {
      disabled: {
        false: null,
        true: "cursor-not-allowed opacity-50 active:pointer-events-none",
      },
    },
  }
);

export const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, disabled = false, iconLeft, iconRight, iconClassName, ...props }, forwardedRef) => {
    return (
      <div className={cn(inputStyles({ disabled }), className)}>
        {iconLeft && <Icon icon={iconLeft} className={iconClassName} />}

        <input ref={forwardedRef} disabled={disabled} className="h-full w-full outline-none" {...props} />

        {iconRight && <Icon icon={iconRight} className={iconClassName} />}
      </div>
    );
  }
);

export default Input;
