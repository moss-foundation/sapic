import { cva } from "class-variance-authority";
import { ButtonHTMLAttributes, forwardRef } from "react";
import { cn } from "@/utils";
import { Icon, Icons } from "./Icon";

export type Button = typeof Button;

export type ButtonIntent = "primary" | "default" | "outlined" | "danger";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  loading?: boolean;
  disabled?: boolean;
  intent?: ButtonIntent;
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
    py-1.25 px-2.5 rounded-md truncate

    transition duration-300 ease-in-out  
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
      intent: {
        primary: `
          background-(--moss-accent) enabled:hover:opacity-80 
          
          border-[color:color-mix(in_srgb,var(--moss-accent)_75%,black)] 
          dark:border-[color:color-mix(in_srgb,var(--moss-accent)_75%,white)] 
          
          text-white
          `,
        default: `
          background-(--moss-button-default)
          enabled:hover:background-[color:color-mix(in_srgb,var(--moss-button-default)_95%,black)]
          dark:enabled:hover:background-[color:color-mix(in_srgb,var(--moss-button-default)_95%,white)]
          
          border-[color:color-mix(in_srgb,var(--moss-button-default)_90%,black)]
          enabled:hover:border-[color:color-mix(in_srgb,var(--moss-button-default)_85%,black)] 
          
          dark:border-[color:color-mix(in_srgb,var(--moss-button-default)_90%,white)]
          
          text-(--moss-button-outlined-foreground)
          `,
        outlined: `
          background-(--moss-button-outlined-background) 
          enabled:hover:background-(--moss-button-outlined-background-hover) 
          
          text-(--moss-button-outlined-foreground) 
          text-black dark:text-white
          
          border-(--moss-button-outlined-border) 
          enabled:hover:border-(--moss-button-outlined-border-hover)`,
        danger: `
          background-(--moss-button-danger)
          enabled:hover:opacity-80
          
          border-[color:color-mix(in_srgb,var(--moss-button-danger)_80%,black)]
          enabled:hover:border-[color:color-mix(in_srgb,var(--moss-button-danger)_75%,black)] 
          
          text-white
          `,
      }
    },
  }
);

const loadingWrapperStyles = cva(`LoadingIcon absolute inset-0 grid place-items-center`, {
  variants: {
    intent: {
      primary: "background-(--moss-accent)",
      default: "background-(--moss-accent)",
      outlined: "background-(--moss-button-outlined-background) rounded-sm",
      danger: "background-(--moss-button-danger)",
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
