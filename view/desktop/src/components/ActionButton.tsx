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
          `background-(--moss-icon-secondary-background) ${customHoverBackground || "hover:background-(--moss-icon-secondary-background-hover)"} active:background-(--moss-icon-secondary-background-active) flex cursor-pointer items-center justify-center rounded-[3px] p-[3px] text-(--moss-icon-secondary-text)`,
          props.disabled &&
            "hover:background-transparent hover:dark:background-transparent cursor-default opacity-50 hover:text-(--moss-icon-secondary-text)"
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
