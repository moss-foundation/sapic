import { ButtonHTMLAttributes, forwardRef } from "react";

import { Icon, Icons } from "@/lib/ui";
import { cn } from "@/utils";

interface ActionButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  icon: Icons;
  className?: string;
  iconClassName?: string;
  customHoverBackground?: string;
  asChild?: boolean;
}

export const ActionButton = forwardRef<HTMLButtonElement, ActionButtonProps>(
  ({ icon, className, iconClassName, customHoverBackground, ...props }, ref) => {
    const buttonContent = (
      <div
        className={cn(
          `background-(--moss-icon-secondary-background) ${customHoverBackground || "hover:background-(--moss-icon-secondary-background-hover)"} active:background-(--moss-icon-secondary-background-active) disabled:hover:background-transparent disabled:hover:dark:background-transparent flex cursor-pointer items-center justify-center rounded-[3px] p-[3px] text-(--moss-icon-secondary-text) disabled:cursor-default disabled:opacity-50 disabled:hover:text-(--moss-icon-secondary-text)`
        )}
      >
        <Icon icon={icon} className={iconClassName} />
      </div>
    );

    return (
      <button ref={ref} className={cn("flex size-[26px] items-center justify-center", className)} {...props}>
        {buttonContent}
      </button>
    );
  }
);

export default ActionButton;
