import React, { ButtonHTMLAttributes, forwardRef } from "react";
import { Icon, type Icons } from "@/components";
import { cn } from "@/utils";

interface IconLabelButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  leftIcon?: Icons;
  rightIcon?: Icons;
  title: string;
  className?: string;
  leftIconClassName?: string;
  rightIconClassName?: string;
  labelClassName?: string;
}

interface LabelProps {
  title: string;
  className?: string;
}

const buttonStyles =
  "group flex h-[22px] cursor-pointer items-center rounded p-1 text-[var(--moss-icon-primary-text)] hover:bg-[var(--moss-icon-primary-background-hover)] disabled:cursor-default disabled:opacity-50";

const ButtonLabel: React.FC<LabelProps> = ({ title, className }) => {
  return (
    <span
      className={cn(
        "text-md overflow-hidden text-ellipsis whitespace-nowrap text-[var(--moss-primary-text)] opacity-100",
        className
      )}
    >
      {title}
    </span>
  );
};

export const IconLabelButton = forwardRef<HTMLButtonElement, IconLabelButtonProps>(
  ({ leftIcon, rightIcon, title, className, leftIconClassName, rightIconClassName, labelClassName, ...props }, ref) => {
    return (
      <button ref={ref} className={cn(buttonStyles, className)} {...props}>
        <div className="flex items-center gap-1">
          {leftIcon && <Icon icon={leftIcon} className={cn("size-4", "mr-0.5", leftIconClassName)} />}
          <ButtonLabel title={title} className={labelClassName} />
          {rightIcon && <Icon icon={rightIcon} className={cn("size-4", rightIconClassName)} />}
        </div>
      </button>
    );
  }
);

export default IconLabelButton;
