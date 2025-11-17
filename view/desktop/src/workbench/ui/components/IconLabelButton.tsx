import { cva } from "class-variance-authority";
import { ButtonHTMLAttributes, forwardRef } from "react";

import { Icon, type Icons } from "@/lib/ui";
import { cn } from "@/utils";

interface IconLabelButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  leftIcon?: Icons;
  rightIcon?: Icons;
  title?: string;
  placeholder?: string;
  className?: string;
  leftIconClassName?: string;
  rightIconClassName?: string;
  labelClassName?: string;
}

const buttonStyles = `
  flex items-center 
  gap-1 px-2 py-1
  cursor-pointer rounded 
  text-(--moss-controls-foreground) 
  hover:background-(--moss-controls-background-hover) 
  disabled:cursor-default 
  disabled:opacity-50
  truncate
`;

const buttonLabelStyles = cva(`text-md truncate text-left`, {
  variants: {
    isPlaceholder: {
      true: "text-(--moss-controls-placeholder)",
      false: "text-(--moss-controls-foreground) text-left",
    },
  },
});

export const IconLabelButton = forwardRef<HTMLButtonElement, IconLabelButtonProps>(
  ({ leftIcon, rightIcon, title, placeholder, className, leftIconClassName, rightIconClassName, ...props }, ref) => {
    return (
      <button ref={ref} className={cn(buttonStyles, className)} {...props}>
        {leftIcon && <Icon icon={leftIcon} className={cn(leftIconClassName)} />}
        <span className={buttonLabelStyles({ isPlaceholder: !title })}>{title ?? placeholder}</span>
        {rightIcon && <Icon icon={rightIcon} className={cn(rightIconClassName)} />}
      </button>
    );
  }
);

export default IconLabelButton;
