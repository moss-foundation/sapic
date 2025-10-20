import { cva } from "class-variance-authority";
import React, { forwardRef, useRef } from "react";

import { useInputResize } from "@/hooks/useInputResize";
import { cn, mergeRefs } from "@/utils";

import Icon, { Icons } from "./Icon";

export interface InputProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, "size"> {
  iconLeft?: Icons;
  iconRight?: Icons;
  iconClassName?: string;
  inputFieldClassName?: string;
  fieldSizing?: "content" | "auto";
  contrast?: boolean;
  intent?: "plain" | "outlined";
}

//prettier-ignore
const inputWrapperStyles = cva(`
    flex items-center w-full gap-2
    border 
    rounded-sm px-2 py-0 

    has-[input:focus-within]:outline-2 
    has-[input:focus-within]:outline-(--moss-primary)   
    has-[input:focus-within]:-outline-offset-2

    has-data-invalid:border-(--moss-error)
  `,
  {
    variants: {
      disabled: {
        false: null,
        true: "cursor-not-allowed opacity-50 active:pointer-events-none pointer-events-none",
      },
      intent: {
        plain: "background-none border-transparent",
        outlined: "background-(--moss-controls-background) border-(--moss-controls-border)",
      },
      contrast: {
        true: "background-(--moss-controls-background-contrast)",
        false: "",
      },
    },
  }
);

const inputStyles = cva(
  `py-[7px] font-normal text-(--moss-controls-foreground) placeholder-(--moss-controls-placeholder)`
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
      contrast = false,
      intent = "plain",
      ...props
    },
    forwardedRef
  ) => {
    const ref = useRef<HTMLInputElement>(null);

    useInputResize({ ref, enabled: fieldSizing === "content" });

    return (
      <div className={cn(inputWrapperStyles({ disabled, contrast, intent }), className)}>
        {iconLeft && <Icon icon={iconLeft} className={iconClassName} />}

        <input
          ref={mergeRefs([ref, forwardedRef])}
          disabled={disabled}
          className={cn(inputStyles(), "h-auto w-full focus-visible:outline-none", inputFieldClassName)}
          {...props}
        />

        {iconRight && <Icon icon={iconRight} className={iconClassName} />}
      </div>
    );
  }
);

export default Input;
