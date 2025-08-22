import { cva } from "class-variance-authority";
import React, { forwardRef, useEffect, useRef } from "react";

import useInputResize from "@/pages/RequestPage/hooks/useInputResize";
import { cn, mergeRefs } from "@/utils";

import Icon, { Icons } from "./Icon";

export interface InputProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, "size"> {
  iconLeft?: Icons;
  iconRight?: Icons;
  iconClassName?: string;
  inputFieldClassName?: string;
  fieldSizing?: "content" | "auto";
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
    {
      className,
      disabled = false,
      iconLeft,
      iconRight,
      iconClassName,
      inputFieldClassName,
      fieldSizing = "auto",
      ...props
    },
    forwardedRef
  ) => {
    const ref = useRef<HTMLInputElement>(null);

    useInputResize({ ref, enabled: fieldSizing === "content" });

    return (
      <div className={cn(inputStyles({ disabled }), className)}>
        {iconLeft && <Icon icon={iconLeft} className={iconClassName} />}

        <input
          ref={mergeRefs([ref, forwardedRef])}
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
