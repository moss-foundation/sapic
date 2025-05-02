import { cva } from "class-variance-authority";
import { ButtonHTMLAttributes, Children, forwardRef, isValidElement } from "react";

import { cn } from "@/utils";

import Icon from "./Icon";

export type Button = typeof Button;

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  loading?: boolean;
  disabled?: boolean;
  href?: string;
  intent?: "primary" | "neutral";
  variant?: "solid" | "outlined";
  size?: "xs" | "sm" | "md" | "lg" | "xl";
}

const buttonRootStyles = cva(
  "relative flex items-center min-w-18 cursor-pointer justify-center rounded-sm transition duration-150 ease-in-out focus-visible:outline-2 focus-visible:outline-offset-2 outline-(--moss-primary)",
  {
    variants: {
      intent: {
        primary: `[--bg-solid:var(--moss-button-primary-solid-background)] [--bg-solid-hover:var(--moss-button-primary-solid-background-hover)] [--bg-solid-active:var(--moss-button-primary-solid-background-active)] [--border-solid:var(--moss-button-primary-solid-border)] [--border-solid-hover:var(--moss-button-primary-solid-border-hover)] [--border-solid-active:var(--moss-button-primary-solid-border-active)] [--text-solid:var(--moss-button-primary-solid-text)] [--bg-outlined:var(--moss-button-primary-outlined-background)] [--bg-outlined-hover:var(--moss-button-primary-outlined-background-hover)] [--bg-outlined-active:var(--moss-button-primary-outlined-background-active)] [--border-outlined:var(--moss-button-primary-outlined-border)] [--border-outlined-hover:var(--moss-button-primary-outlined-border-hover)] [--border-outlined-active:var(--moss-button-primary-outlined-border-active)] [--text-outlined:var(--moss-button-primary-outlined-text)]`,
        neutral: `[--bg-solid:var(--moss-button-neutral-solid-background)] [--bg-solid-hover:var(--moss-button-neutral-solid-background-hover)] [--bg-solid-active:var(--moss-button-neutral-solid-background-active)] [--border-solid:var(--moss-button-neutral-solid-border)] [--border-solid-hover:var(--moss-button-neutral-solid-border-hover)] [--border-solid-active:var(--moss-button-neutral-solid-border-active)] [--text-solid:var(--moss-button-neutral-solid-text)] [--bg-outlined:var(--moss-button-neutral-outlined-background)] [--bg-outlined-hover:var(--moss-button-neutral-outlined-background-hover)] [--bg-outlined-active:var(--moss-button-neutral-outlined-background-active)] [--border-outlined:var(--moss-button-neutral-outlined-border)] [--border-outlined-hover:var(--moss-button-neutral-outlined-border-hover)] [--border-outlined-active:var(--moss-button-neutral-outlined-border-active)] [--text-outlined:var(--moss-button-neutral-outlined-text)]`,
      },
      variant: {
        solid: `    background-(--bg-solid)    hover:background-(--bg-solid-hover)    active:background-(--bg-solid-active)    text-(--text-solid)`,
        outlined: ` background-(--bg-outlined) hover:background-(--bg-outlined-hover) active:background-(--bg-outlined-active) text-(--text-outlined) border border-(--border-outlined) hover:border-(--border-outlined-hover) active:border-(--border-outlined-active)`,
      },
      size: {
        "xs": "h-[22px]",
        "sm": "h-[26px]",
        "md": "h-[28px]",
        "lg": "h-[34px]",
        "xl": "h-[38px]",
      },
      isDisabled: {
        false: null,
        true: "cursor-not-allowed hover:brightness-100 active:brightness-100 background-(--moss-button-background-disabled) border border-(--moss-button-border-disabled) text-(--moss-button-text-disabled) hover:background-(--moss-button-background-disabled-hover) active:background-(--moss-button-background-disabled-active) hover:border-(--moss-button-border-disabled-hover) active:border-(--moss-button-border-disabled-active)",
      },
      loading: {
        false: null,
        true: "[&>:not(.LoadingIcon)]:opacity-0 cursor-progress ",
      },
      iconOnly: {
        false: "notOnlyIcon",
        true: "iconOnly",
      },
    },
    compoundVariants: [
      {
        iconOnly: true,
        size: "xs",
        className: "px-1.5",
      },
      {
        iconOnly: false,
        size: "xs",
        className: "px-3",
      },
      {
        iconOnly: true,
        size: "sm",
        className: "px-2",
      },
      {
        iconOnly: false,
        size: "sm",
        className: "px-3.5",
      },
      {
        iconOnly: true,
        size: "md",
        className: "px-2.5",
      },
      {
        iconOnly: false,
        size: "md",
        className: " px-4",
      },
      {
        iconOnly: true,
        size: "lg",
        className: "px-3",
      },
      {
        iconOnly: false,
        size: "lg",
        className: " px-5",
      },
      {
        iconOnly: true,
        size: "xl",
        className: "px-4",
      },
      {
        iconOnly: false,
        size: "xl",
        className: "px-6",
      },
    ],
  }
);

const loadingIconStyles = cva("animate-spin", {
  variants: {
    size: {
      "xs": "size-2",
      "sm": "size-3",
      "md": "size-5",
      "lg": "size-7",
      "xl": "size-8",
    },
  },
});

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  (
    { className, variant = "solid", size = "md", disabled, loading, href, children, intent = "primary", ...props },
    forwardedRef
  ) => {
    const iconOnly =
      Children.toArray(children).length === 1 &&
      Children.toArray(children).some((child) => isValidElement(child) && child.type === Icon);

    const content = typeof children === "string" ? <span>{children}</span> : children;

    const isDisabled = disabled || loading;

    return (
      <button
        ref={forwardedRef}
        className={cn(buttonRootStyles({ size, isDisabled, loading, iconOnly, intent, variant }), className)}
        disabled={isDisabled}
        {...props}
      >
        {content}

        {loading && (
          <div className="LoadingIcon absolute inset-0 grid place-items-center">
            <Icon icon="LoaderTailus" className={cn(loadingIconStyles({ size }))} />
          </div>
        )}
      </button>
    );
  }
);

export default Button;
