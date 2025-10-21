import { cva } from "class-variance-authority";
import { ButtonHTMLAttributes, forwardRef } from "react";

import { cn } from "@/utils";

import { Icon, Icons } from "./Icon";

export type Button = typeof Button;

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  loading?: boolean;
  disabled?: boolean;
  intent?: "primary" | "outlined" | "danger";
  fullWidth?: boolean;
  iconLeft?: Icons;
  iconRight?: Icons;
  icon?: Icons;
  iconClassName?: string;
}

//prettier-ignore
const buttonRootStyles = cva(`
    relative 
    flex items-center justify-center  gap-1
    min-w-18 
    py-1.5 px-1 rounded-sm truncate

    transition duration-150 ease-in-out  
    cursor-pointer  
  
    border

    enabled:active:brightness-90

    focus-visible:outline-(--moss-accent)
    focus-visible:outline-offset-2
    focus-visible:outline-2
`,
  {
    variants: {
      disabled: {
        false: null,
        true: "background-(--moss-background-disabled)! text-(--moss-foreground-disabled)! border-(--moss-border-disabled)! cursor-not-allowed!",
      },
      fullWidth: {
        false: "min-w-18",
        true: "w-full",
      },
      loading: {
        false: null,
        true: "cursor-progress [&>:not(.LoadingIcon)]:opacity-0",
      },
      iconOnly: {
        false: null,
        true: "min-w-auto! py-1.5 px-1.5",
      },
      intent:{
        primary:  "background-(--moss-accent)  enabled:hover:background-(--moss-accent-hover) enabled:hover:border-(--moss-accent-hover)   text-(--moss-button-primary-foreground)  border-(--moss-accent)" ,
        outlined: "background-(--moss-button-outlined-background) enabled:hover:background-(--moss-button-outlined-background-hover) text-(--moss-button-outlined-foreground)  border-(--moss-button-outlined-border) enabled:hover:border-(--moss-button-outlined-border-hover) ",
        danger:   "background-(--moss-button-danger-background)   enabled:hover:background-(--moss-button-danger-background-hover)   text-(--moss-button-danger-foreground)  border-transparent",
      }
    },
  }
);

const loadingWrapperStyles = cva(`LoadingIcon absolute inset-0 grid place-items-center`, {
  variants: {
    intent: {
      primary: "background-(--moss-accent)",
      outlined: "background-(--moss-button-outlined-background) rounded-sm",
      danger: "background-(--moss-button-danger-background)",
    },
  },
});

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      className,
      disabled,
      loading,
      children,
      iconLeft,
      iconRight,
      icon,
      iconClassName,
      intent = "primary",
      fullWidth = false,
      ...props
    },
    forwardedRef
  ) => {
    const content = icon ? <Icon icon={icon} /> : children;
    const isDisabled = disabled || loading;

    const iconOnly = !!icon;

    return (
      <button
        ref={forwardedRef}
        className={buttonRootStyles({ disabled, loading, intent, fullWidth, iconOnly, className })}
        disabled={isDisabled}
        type="button"
        {...props}
      >
        {iconLeft && <Icon icon={iconLeft} className={iconClassName} />}
        <div className="truncate">{content}</div>
        {iconRight && <Icon icon={iconRight} className={iconClassName} />}

        {loading && (
          <div className={loadingWrapperStyles({ intent })}>
            <Icon icon="Loader" className={cn("animate-spin", iconClassName)} />
          </div>
        )}
      </button>
    );
  }
);

export default Button;
