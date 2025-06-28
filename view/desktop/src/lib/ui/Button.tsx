import { cva } from "class-variance-authority";
import { ButtonHTMLAttributes, Children, forwardRef, isValidElement } from "react";

import { cn } from "@/utils";

import { Icon } from "./Icon";

export type Button = typeof Button;

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  loading?: boolean;
  disabled?: boolean;
}

const buttonRootStyles = cva(
  "relative flex min-w-18 cursor-pointer items-center justify-center rounded-sm transition duration-150 ease-in-out focus-visible:outline-2 focus-visible:outline-offset-2",
  {
    variants: {
      isDisabled: {
        false: null,
        true: "cursor-not-allowed border",
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
  ({ className, disabled, loading, children, ...props }, forwardedRef) => {
    const iconOnly =
      Children.toArray(children).length === 1 &&
      Children.toArray(children).some((child) => isValidElement(child) && child.type === Icon);

    const content = typeof children === "string" ? <span>{children}</span> : children;

    const isDisabled = disabled || loading;

    return (
      <button
        ref={forwardedRef}
        className={cn(buttonRootStyles({ isDisabled, loading, iconOnly }), className)}
        disabled={isDisabled}
        type="button"
        {...props}
      >
        {content}

        {loading && (
          <div className="LoadingIcon absolute inset-0 grid place-items-center">
            <Icon icon="Loader" className={cn(loadingIconStyles())} />
          </div>
        )}
      </button>
    );
  }
);

export default Button;
