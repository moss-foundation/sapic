import { cva } from "class-variance-authority";
import React from "react";

import { cn } from "@/utils";

import Icon, { Icons } from "./Icon";

export interface InputProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, "size"> {
  variant?: "plain" | "outlined";
  size?: "xs" | "sm" | "md";
  iconLeft?: Icons;
  iconRight?: Icons;
}

const inputStyles = cva("w-full flex gap-2 items-center peer placeholder-(--moss-controls-placeholder)", {
  variants: {
    variant: {
      plain: `
          text-(--moss-controls-plain-text)
          py-0 rounded-sm
          background-(--moss-input-bg-plain)
          transition-[outline] outline-none
          has-[input:data-[invalid]]:text-(--moss-error)
        `,
      outlined: `
          text-(--moss-controls-outlined-text)
          rounded-sm
          background-(--moss-controls-outlined-bg)
          transition-[outline]
          has-[input:focus-within]:outline-2
          has-[input:focus-within]:outline-(--moss-primary)
          has-[input:focus-within]:-outline-offset-1
          border border-(--moss-controls-outlined-border)
          has-[input:data-[invalid]]:border-(--moss-error)
          has-[input:focus-within]:[data-[invalid]]:outline-(--moss-error)
        `,
    },
    size: {
      xs: "h-6 px-2.5",
      sm: "h-7 px-2.5",
      md: "h-9 px-3",
      lg: "h-10 px-4 text-base",
      xl: "h-12 px-5 text-base",
    },
    disabled: {
      false: null,
      true: "cursor-not-allowed opacity-50 active:pointer-events-none",
    },
  },
});

const iconsStyles = cva("", {
  variants: {
    size: {
      xs: "size-4",
      sm: "size-4",
      md: "size-4",
    },
  },
});

export const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ variant = "plain", className, size = "md", disabled = false, iconLeft, iconRight, ...props }, forwardedRef) => {
    return (
      <div className={cn(inputStyles({ variant, disabled, size }), className)}>
        {iconLeft && (
          <Icon icon={iconLeft} className={cn(iconsStyles({ size }), "text-(--moss-controls-placeholder)")} />
        )}

        <input ref={forwardedRef} disabled={disabled} {...props} className="h-full w-full outline-none" />

        {iconRight && (
          <Icon icon={iconRight} className={cn(iconsStyles({ size }), "text-(--moss-controls-placeholder)")} />
        )}
      </div>
    );
  }
);

export default Input;
