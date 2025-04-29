import React, { ButtonHTMLAttributes, forwardRef } from "react";
import { Icon, type Icons } from "@/components";
import { cn } from "@/utils";

interface IconLabelButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  leftIcon: Icons;
  rightIcon?: Icons;
  title: string;
  className?: string;
}

interface LabelProps {
  title: string;
  className?: string;
}

const ButtonLabel: React.FC<LabelProps> = ({ title, className }) => {
  return (
    <span
      className={cn(
        "overflow-hidden text-xs text-ellipsis whitespace-nowrap text-[var(--moss-not-selected-item-color)] opacity-100",
        className
      )}
    >
      {title}
    </span>
  );
};

export const IconLabelButton = forwardRef<HTMLButtonElement, IconLabelButtonProps>(
  ({ leftIcon, rightIcon, title, className, ...props }, ref) => {
    return (
      <button
        ref={ref}
        className={cn(
          "group flex h-[22px] cursor-pointer items-center rounded p-1 text-[var(--moss-icon-primary-text)]",
          "hover:bg-[var(--moss-icon-primary-background-hover)]",
          "disabled:cursor-default disabled:opacity-50",
          className
        )}
        {...props}
      >
        <div className="flex items-center gap-1">
          <Icon icon={leftIcon} className="mr-0.5" />
          <ButtonLabel title={title} />
          {rightIcon && <Icon icon={rightIcon} />}
        </div>
      </button>
    );
  }
);

export default IconLabelButton;
