import React, { ButtonHTMLAttributes, forwardRef } from "react";
import { Icon, type Icons } from "@/components";
import { cn } from "@/utils";

interface IconLabelButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  leftIcon?: Icons;
  rightIcon?: Icons;
  title: string;
  placeholder?: string;
  className?: string;
  leftIconClassName?: string;
  rightIconClassName?: string;
  labelClassName?: string;
  placeholderClassName?: string;
  compact?: boolean;
  showPlaceholder?: boolean;
}

interface LabelProps {
  title: string;
  placeholder?: string;
  className?: string;
  placeholderClassName?: string;
  showPlaceholder?: boolean;
}

const buttonStyles =
  "group flex h-[22px] cursor-pointer items-center rounded p-[3px] text-[var(--moss-icon-primary-text)] hover:bg-[var(--moss-icon-primary-background-hover)] disabled:cursor-default disabled:opacity-50";

const ButtonLabel: React.FC<LabelProps> = ({
  title,
  placeholder,
  className,
  placeholderClassName,
  showPlaceholder,
}) => {
  if (showPlaceholder && placeholder) {
    return (
      <span
        className={cn(
          "text-md overflow-hidden text-ellipsis whitespace-nowrap text-[var(--moss-not-selected-item-color)]",
          placeholderClassName
        )}
      >
        {placeholder}
      </span>
    );
  }

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
  (
    {
      leftIcon,
      rightIcon,
      title,
      placeholder,
      className,
      leftIconClassName,
      rightIconClassName,
      labelClassName,
      placeholderClassName,
      compact = false,
      showPlaceholder = false,
      ...props
    },
    ref
  ) => {
    return (
      <button ref={ref} className={cn(buttonStyles, className)} {...props}>
        <div className={compact ? "flex items-center gap-0.5" : "flex items-center gap-1 px-1"}>
          {leftIcon && <Icon icon={leftIcon} className={cn("size-4", leftIconClassName)} />}
          {!compact && (
            <ButtonLabel
              title={title}
              placeholder={placeholder}
              className={cn("mx-0.5", labelClassName)}
              placeholderClassName={cn("mx-0.5", placeholderClassName)}
              showPlaceholder={showPlaceholder}
            />
          )}
          {rightIcon && <Icon icon={rightIcon} className={cn("size-4", rightIconClassName)} />}
        </div>
      </button>
    );
  }
);

export default IconLabelButton;
