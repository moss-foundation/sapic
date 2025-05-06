import { cva } from "class-variance-authority";
import React from "react";

import { cn } from "@/utils";

import Icon, { Icons } from "../lib/ui/Icon";

export interface InputProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, "size"> {
  variant?: "plain" | "outlined";
  size?: "xs" | "sm" | "md";
  iconLeft?: Icons;
  iconRight?: Icons;
}

const inputStyles = cva("peer flex w-full items-center gap-2 font-medium placeholder-(--moss-controls-placeholder)", {
  variants: {
    variant: {
      plain: `background-(--moss-input-bg-plain) rounded-sm py-0 text-(--moss-controls-plain-text) transition-[outline] has-data-invalid:text-(--moss-error) has-[input:focus-within]:outline has-[input:focus-within]:-outline-offset-1 has-[input:focus-within]:outline-(--moss-primary)`,
      outlined: `background-(--moss-controls-outlined-bg) rounded-sm border border-(--moss-controls-outlined-border) text-(--moss-controls-outlined-text) transition-[outline] has-data-invalid:border-(--moss-error) has-[input:focus-within]:outline-2 has-[input:focus-within]:-outline-offset-1 has-[input:focus-within]:outline-(--moss-primary) has-[input:focus-within]:has-data-invalid:outline-(--moss-error)`,
    },
    size: {
      xs: "h-6 px-1.5",
      sm: "h-7 px-2",
      md: "h-9 px-2",
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
  ({ variant = "plain", className, size = "sm", disabled = false, iconLeft, iconRight, ...props }, forwardedRef) => {
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
