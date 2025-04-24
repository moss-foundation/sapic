import { cva } from "class-variance-authority";
import { Children, forwardRef, HTMLAttributes, isValidElement } from "react";

import { cn } from "@/utils";

import Icon from "./Icon";

export type Button = typeof Button;

export interface ButtonProps extends HTMLAttributes<HTMLButtonElement | HTMLAnchorElement> {
  loading?: boolean;
  disabled?: boolean;
  href?: string;
  intent?: "primary" | "danger" | "neutral";
  variant?: "solid" | "outlined" | "soft" | "ghost";
  size?: "xs" | "sm" | "md" | "lg" | "xl";
}

const buttonRootStyles = cva(
  "relative flex items-center cursor-pointer justify-center rounded-sm transition duration-150 ease-in-out focus-visible:outline-2 focus-visible:outline-offset-2 outline-blue-600",
  {
    variants: {
      intent: {
        primary: `[--bg-solid:var(--moss-button-primary-solid-background)] [--border-solid:var(--moss-button-primary-solid-border)] [--text-solid:var(--moss-button-primary-solid-text)] [--bg-outlined:var(--moss-button-primary-outlined-background)] [--border-outlined:var(--moss-button-primary-outlined-border)] [--text-outlined:var(--moss-button-primary-outlined-text)] [--bg-soft:var(--moss-button-primary-soft-background)] [--border-soft:var(--moss-button-primary-soft-border)] [--text-soft:var(--moss-button-primary-soft-text)] [--bg-ghost:var(--moss-button-primary-ghost-background)] [--border-ghost:var(--moss-button-primary-ghost-border)] [--text-ghost:var(--moss-button-primary-ghost-text)] [--boxShadow-solid:var(--moss-button-primary-solid-boxShadow)] [--boxShadow-outlined:var(--moss-button-primary-outlined-boxShadow)] [--boxShadow-soft:var(--moss-button-primary-soft-boxShadow)] [--boxShadow-ghost:var(--moss-button-primary-ghost-boxShadow)]`,
        danger: ` [--bg-solid:var(--moss-button-danger-solid-background)]  [--border-solid:var(--moss-button-danger-solid-border)]  [--text-solid:var(--moss-button-danger-solid-text)]  [--bg-outlined:var(--moss-button-danger-outlined-background)]  [--border-outlined:var(--moss-button-danger-outlined-border)]  [--text-outlined:var(--moss-button-danger-outlined-text)]  [--bg-soft:var(--moss-button-danger-soft-background)]  [--border-soft:var(--moss-button-danger-soft-border)]  [--text-soft:var(--moss-button-danger-soft-text)]  [--bg-ghost:var(--moss-button-danger-ghost-background)]  [--border-ghost:var(--moss-button-danger-ghost-border)]  [--text-ghost:var(--moss-button-danger-ghost-text)]  [--boxShadow-solid:var(--moss-button-danger-solid-boxShadow)]  [--boxShadow-outlined:var(--moss-button-danger-outlined-boxShadow)]  [--boxShadow-soft:var(--moss-button-danger-soft-boxShadow)]  [--boxShadow-ghost:var(--moss-button-danger-ghost-boxShadow)]`,
        neutral: `[--bg-solid:var(--moss-button-neutral-solid-background)] [--border-solid:var(--moss-button-neutral-solid-border)] [--text-solid:var(--moss-button-neutral-solid-text)] [--bg-outlined:var(--moss-button-neutral-outlined-background)] [--border-outlined:var(--moss-button-neutral-outlined-border)] [--text-outlined:var(--moss-button-neutral-outlined-text)] [--bg-soft:var(--moss-button-neutral-soft-background)] [--border-soft:var(--moss-button-neutral-soft-border)] [--text-soft:var(--moss-button-neutral-soft-text)] [--bg-ghost:var(--moss-button-neutral-ghost-background)] [--border-ghost:var(--moss-button-neutral-ghost-border)] [--text-ghost:var(--moss-button-neutral-ghost-text)] [--boxShadow-solid:var(--moss-button-neutral-solid-boxShadow)] [--boxShadow-outlined:var(--moss-button-neutral-outlined-boxShadow)] [--boxShadow-soft:var(--moss-button-neutral-soft-boxShadow)] [--boxShadow-ghost:var(--moss-button-neutral-ghost-boxShadow)]`,
      },
      variant: {
        solid: `   background-(--bg-solid)    text-(--text-solid)    [box-shadow:var(--boxShadow-solid)]    dark:border-t dark:border-(--border-solid) hover:brightness-110 active:brightness-95 `,
        outlined: `background-(--bg-outlined) text-(--text-outlined) [box-shadow:var(--boxShadow-outlined)] hover:brightness-[0.98] active:brightness-100 dark:hover:brightness-150 dark:active:background-(--bg-outlined)/70`,
        soft: `    background-(--bg-soft)     text-(--text-soft)     [box-shadow:var(--boxShadow-soft)]     hover:brightness-95     active:brightness-105 dark:hover:brightness-120 dark:active:background-(--bg-soft)/70`,
        ghost: `   background-transparent     text-(--text-ghost)    [box-shadow:var(--boxShadow-ghost)]    dark:border-(--border-ghost) hover:background-(--bg-ghost) hover:[box-shadow:var(--border-ghost)_0px_0px_0px_1px] active:brightness-150 `,
      },
      size: {
        "xs": "h-[22px]",
        "sm": "h-[26px]",
        "md": "h-[28px]",
        "lg": "h-[34px]",
        "xl": "h-[38px]",
      },
      disabled: {
        false: null,
        true: "grayscale-70 cursor-not-allowed hover:brightness-100 active:brightness-100 pointer-events-none",
      },
      loading: {
        false: null,
        true: "[&>:not(.LoadingIcon)]:opacity-0 pointer-events-none",
      },
      Component: {
        a: "max-w-max",
        button: null,
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
      "xs": "size-5",
      "sm": "size-6",
      "md": "size-7",
      "lg": "size-8",
      "xl": "size-9",
    },
  },
});

export const Button = forwardRef<HTMLButtonElement & HTMLAnchorElement, ButtonProps>(
  (
    { className, variant = "solid", size = "md", disabled, loading, href, children, intent = "primary", ...props },
    forwardedRef
  ) => {
    const Component = href ? "a" : "button";
    const iconOnly =
      Children.toArray(children).length === 1 &&
      Children.toArray(children).some((child) => isValidElement(child) && child.type === Icon);

    const content = typeof children === "string" ? <span>{children}</span> : children;

    return (
      <Component
        ref={forwardedRef}
        type={Component === "button" ? "button" : undefined}
        href={disabled || loading ? undefined : href}
        className={cn(buttonRootStyles({ size, disabled, loading, Component, iconOnly, intent, variant }), className)}
        disabled={disabled || loading}
        {...props}
      >
        {content}

        {loading && (
          <div className="LoadingIcon absolute inset-0 grid place-items-center">
            <Icon icon="LoaderTailus" className={cn(loadingIconStyles({ size }))} />
          </div>
        )}
      </Component>
    );
  }
);

export default Button;
