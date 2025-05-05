import { cva } from "class-variance-authority";
import { ButtonHTMLAttributes, Children, forwardRef, isValidElement } from "react";

import { Icon } from "@/components";
import { cn } from "@/utils";

export type Button = typeof Button;

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  loading?: boolean;
  disabled?: boolean;
  href?: string;
  size?: "xs" | "sm" | "md" | "lg" | "xl";
}

const buttonRootStyles = cva(
  "relative flex min-w-18 cursor-pointer items-center justify-center rounded-sm outline-(--moss-primary) transition duration-150 ease-in-out focus-visible:outline-2 focus-visible:outline-offset-2",
  {
    variants: {
      size: {
        "xs": "h-[22px]",
        "sm": "h-[26px]",
        "md": "h-[28px]",
        "lg": "h-[34px]",
        "xl": "h-[38px]",
      },
      isDisabled: {
        false: null,
        true: "cursor-not-allowed border hover:brightness-100 active:brightness-100",
      },
      loading: {
        false: null,
        true: "cursor-progress [&>:not(.LoadingIcon)]:opacity-0",
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
        className: "px-4",
      },
      {
        iconOnly: true,
        size: "lg",
        className: "px-3",
      },
      {
        iconOnly: false,
        size: "lg",
        className: "px-5",
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
  ({ className, size = "md", disabled, loading, href, children, ...props }, forwardedRef) => {
    const iconOnly =
      Children.toArray(children).length === 1 &&
      Children.toArray(children).some((child) => isValidElement(child) && child.type === Icon);

    const content = typeof children === "string" ? <span>{children}</span> : children;

    const isDisabled = disabled || loading;

    return (
      <button
        ref={forwardedRef}
        className={cn(buttonRootStyles({ size, isDisabled, loading, iconOnly }), className)}
        disabled={isDisabled}
        {...props}
      >
        {content}

        {loading && (
          <div className="LoadingIcon absolute inset-0 grid place-items-center">
            <Icon icon="Loader" className={cn(loadingIconStyles({ size }))} />
          </div>
        )}
      </button>
    );
  }
);

export default Button;
