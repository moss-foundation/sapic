import { cva } from "class-variance-authority";
import React, { forwardRef, useRef } from "react";

import { useInputResize } from "@/hooks/useInputResize";
import { cn, mergeRefs } from "@/utils";

import Icon, { Icons } from "./Icon";

export interface InputProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, "size"> {
  iconLeft?: Icons;
  shortcut?: string;
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
    rounded-md pl-2 pr-[4px] 

    has-[input:focus-within]:outline-2 
    has-[input:focus-within]:outline-(--moss-accent)   
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
  `text-(--moss-controls-foreground) placeholder-(--moss-controls-placeholder) h-auto w-full py-[5px] font-normal focus-visible:outline-none`
);

const shortcutStyles = cva(
  `background-(--moss-controls-shortcut-background) text-(--moss-controls-shortcut-foreground) shrink-0 rounded-sm px-1 py-0.5 font-semibold`
);

export const Input = forwardRef<HTMLInputElement, InputProps>(
  (
    {
      className,
      disabled = false,
      iconLeft,
      shortcut,
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
      <div className={inputWrapperStyles({ disabled, contrast, intent, className })}>
        {iconLeft && <Icon icon={iconLeft} className={iconClassName} />}

        <input
          ref={mergeRefs([ref, forwardedRef])}
          disabled={disabled}
          className={cn(inputStyles(), inputFieldClassName)}
          {...props}
        />

        {shortcut && <span className={shortcutStyles()}>{shortcut}</span>}
      </div>
    );
  }
);

export default Input;
