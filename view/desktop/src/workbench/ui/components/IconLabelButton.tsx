import { cva } from "class-variance-authority";
import { ButtonHTMLAttributes, forwardRef } from "react";

import { Icon, type Icons } from "@/lib/ui";
import { cn } from "@/utils";

interface IconLabelButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  leftIcon?: Icons;
  rightIcon?: Icons;
  monogram?: string;
  title?: string;
  placeholder?: string;
  className?: string;
  leftIconClassName?: string;
  rightIconClassName?: string;
}

const buttonStyles = `
  flex items-center 
  gap-1 px-1 py-1
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
  (
    { leftIcon, rightIcon, monogram, title, placeholder, className, leftIconClassName, rightIconClassName, ...props },
    ref
  ) => {
    return (
      <button ref={ref} className={cn(buttonStyles, className)} {...props}>
        {monogram ? (
          <div className="size-4.5 background-(--moss-accent) mr-0.25 flex shrink-0 cursor-pointer items-center justify-center rounded-sm text-xs font-medium text-white">
            {monogram}
          </div>
        ) : (
          leftIcon && <Icon icon={leftIcon} className={cn(leftIconClassName)} />
        )}
        <span className={buttonLabelStyles({ isPlaceholder: !title })}>{title ?? placeholder}</span>
        {rightIcon && <Icon icon={rightIcon} className={cn(rightIconClassName)} />}
      </button>
    );
  }
);

export default IconLabelButton;
