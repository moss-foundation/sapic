import { cva } from "class-variance-authority";
import React, { forwardRef } from "react";

import { cn } from "@/utils";

import Icon, { Icons } from "./Icon";

export interface InputProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, "size"> {
  iconLeft?: Icons;
  iconRight?: Icons;
  iconClassName?: string;
  inputFieldClassName?: string;
}

//prettier-ignore
const inputStyles = cva(`
    flex items-center w-full gap-2
    rounded-sm py-0 font-medium transition-shadow 
    has-[input:focus-within]:outline-2 
    has-[input:focus-within]:outline-(--moss-primary)   
    has-[input:focus-within]:-outline-offset-2
  `,
  {
    variants: {
      disabled: {
        false: null,
        true: "cursor-not-allowed opacity-50 active:pointer-events-none pointer-events-none",
      },
    },
  }
);

export const Input = forwardRef<HTMLInputElement, InputProps>(
  (
    { className, disabled = false, iconLeft, iconRight, iconClassName, inputFieldClassName, ...props },
    forwardedRef
  ) => {
    return (
      <div className={cn(inputStyles({ disabled }), className)}>
        {iconLeft && <Icon icon={iconLeft} className={iconClassName} />}

        <input
          ref={forwardedRef}
          disabled={disabled}
          className={cn("h-auto w-full focus-visible:outline-none", inputFieldClassName)}
          {...props}
        />

        {iconRight && <Icon icon={iconRight} className={iconClassName} />}
      </div>
    );
  }
);

export default Input;
