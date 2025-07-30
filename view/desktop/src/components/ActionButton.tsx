import { ButtonHTMLAttributes, forwardRef } from "react";

import { Icon, Icons } from "@/lib/ui";
import { cn } from "@/utils";

interface ActionButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  icon: Icons;
  size?: "small" | "medium";
  className?: string;
  iconClassName?: string;
  customHoverBackground?: string;
  asChild?: boolean;
}

export const ActionButton = forwardRef<HTMLButtonElement, ActionButtonProps>(
  ({ icon, className, iconClassName, customHoverBackground, size = "medium", ...props }, ref) => {
    const buttonContent = (
      <div
        className={cn(
          `background-(--moss-icon-secondary-background) active:background-(--moss-icon-secondary-background-active) flex cursor-pointer items-center justify-center rounded-[3px] text-(--moss-icon-secondary-text)`,
          {
            "hover:background-(--moss-icon-secondary-background-hover)": !customHoverBackground,
            "p-[1px]": size === "small",
            "p-[3px]": size === "medium",
            "hover:background-transparent cursor-default opacity-50 hover:text-(--moss-icon-secondary-text)":
              props.disabled,
          },
          customHoverBackground
        )}
      >
        <Icon icon={icon} className={cn(iconClassName)} />
      </div>
    );

    return (
      <button
        ref={ref}
        className={cn(
          "grid place-items-center",
          {
            "size-[18px]": size === "small",
            "size-[26px]": size === "medium",
          },
          className
        )}
        {...props}
      >
        {buttonContent}
      </button>
    );
  }
);

export default ActionButton;
