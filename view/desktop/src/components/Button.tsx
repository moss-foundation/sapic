import { cva } from "class-variance-authority";
import { Children, forwardRef, HTMLAttributes, isValidElement } from "react";

import { cn } from "@/utils";

import Icon from "./Icon";

export type Button = typeof Button;

export interface ButtonProps extends HTMLAttributes<HTMLButtonElement | HTMLAnchorElement> {
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
        primary: `[--bg-solid:var(--moss-button-primary-solid-background)] [--border-solid:var(--moss-button-primary-solid-border)] [--text-solid:var(--moss-button-primary-solid-text)] [--bg-outlined:var(--moss-button-primary-outlined-background)] [--border-outlined:var(--moss-button-primary-outlined-border)] [--text-outlined:var(--moss-button-primary-outlined-text)] [--boxShadow-solid:var(--moss-button-primary-solid-boxShadow)] [--boxShadow-outlined:var(--moss-button-primary-outlined-boxShadow)] [--bg-solid-disabled:var(--moss-button-primary-solid-background-disabled)] [--border-solid-disabled:var(--moss-button-primary-solid-border-disabled)] [--text-solid-disabled:var(--moss-button-primary-solid-text-disabled)] [--bg-outlined-disabled:var(--moss-button-primary-outlined-background-disabled)] [--border-outlined-disabled:var(--moss-button-primary-outlined-border-disabled)] [--text-outlined-disabled:var(--moss-button-primary-outlined-text-disabled)] [--boxShadow-solid-disabled:var(--moss-button-primary-solid-boxShadow-disabled)] [--boxShadow-outlined-disabled:var(--moss-button-primary-outlined-boxShadow-disabled)]`,
        neutral: `[--bg-solid:var(--moss-button-neutral-solid-background)] [--border-solid:var(--moss-button-neutral-solid-border)] [--text-solid:var(--moss-button-neutral-solid-text)] [--bg-outlined:var(--moss-button-neutral-outlined-background)] [--border-outlined:var(--moss-button-neutral-outlined-border)] [--text-outlined:var(--moss-button-neutral-outlined-text)] [--boxShadow-solid:var(--moss-button-neutral-solid-boxShadow)] [--boxShadow-outlined:var(--moss-button-neutral-outlined-boxShadow)] [--bg-solid-disabled:var(--moss-button-neutral-solid-background-disabled)] [--border-solid-disabled:var(--moss-button-neutral-solid-border-disabled)] [--text-solid-disabled:var(--moss-button-neutral-solid-text-disabled)] [--bg-outlined-disabled:var(--moss-button-neutral-outlined-background-disabled)] [--border-outlined-disabled:var(--moss-button-neutral-outlined-border-disabled)] [--text-outlined-disabled:var(--moss-button-neutral-outlined-text-disabled)] [--boxShadow-solid-disabled:var(--moss-button-neutral-solid-boxShadow-disabled)] [--boxShadow-outlined-disabled:var(--moss-button-neutral-outlined-boxShadow-disabled)]`,
      },
      variant: {
        solid: `   hover:brightness-110 active:brightness-95  background-(--bg-solid)       text-(--text-solid)    [box-shadow:var(--boxShadow-solid)]     disabled:background-(--bg-solid-disabled) disabled:text-(--text-solid-disabled) disabled:[box-shadow:var(--boxShadow-solid-disabled)]`,
        outlined: `hover:brightness-[0.98] active:brightness-100 background-(--bg-outlined) text-(--text-outlined) [box-shadow:var(--boxShadow-outlined)]  disabled:background-(--bg-outlined-disabled) disabled:text-(--text-outlined-disabled) disabled:[box-shadow:var(--boxShadow-outlined-disabled)]`,
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
        true: "cursor-not-allowed hover:brightness-100 active:brightness-100",
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
