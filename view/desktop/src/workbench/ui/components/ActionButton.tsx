import { cva } from "class-variance-authority";
import { ButtonHTMLAttributes, forwardRef } from "react";

import { Icon, Icons } from "@/lib/ui";
import { cn } from "@/utils";

interface ActionButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  icon: Icons;
  className?: string;
  iconClassName?: string;
  asChild?: boolean;
  hoverVariant?: "default" | "list";
  forceHoverStyles?: boolean;
}
//prettier-ignore
const actionButtonStyles = cva(`
  grid place-items-center items-center justify-center 
  rounded-[3px] p-[3px] 
  cursor-pointer

  text-(--moss-toolbarItem-foreground)
  `,
  {
    variants: {
      disabled: {
        true: "hover:background-transparent cursor-not-allowed opacity-50 hover:text-(--moss-foreground-disabled)",
      },
      hoverVariant: {
        default: "hover:background-(--moss-toolbarItem-background-hover)",
        list: "hover:background-(--moss-list-toolbarItem-background-hover)",
      },
      forceHoverStyles: {
        true: "background-(--moss-toolbarItem-background-hover)",
        false: "",
      },
    },
    compoundVariants: [
      {
        forceHoverStyles: true,
        hoverVariant: "default",
        class: "background-(--moss-toolbarItem-background-hover)",
      },
      {
        forceHoverStyles: true,
        hoverVariant: "list", 
        class: "background-(--moss-list-toolbarItem-background-hover)",
      },
    ],
  }
);

export const ActionButton = forwardRef<HTMLButtonElement, ActionButtonProps>(
  ({ className, disabled, icon, hoverVariant = "default", iconClassName, forceHoverStyles = false, ...props }, ref) => {
    return (
      <button
        ref={ref}
        className={actionButtonStyles({ disabled, hoverVariant, className, forceHoverStyles })}
        {...props}
      >
        <Icon icon={icon} className={cn(iconClassName)} />
      </button>
    );
  }
);

export default ActionButton;
