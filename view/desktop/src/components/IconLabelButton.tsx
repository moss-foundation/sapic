import React, { ButtonHTMLAttributes, forwardRef } from "react";
import { Icon, type Icons } from "@/components";
import { cn } from "@/utils";
import { cva } from "class-variance-authority";

interface IconLabelButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  leftIcon: Icons;
  rightIcon?: Icons;
  title: string;
  className?: string;
  leftIconClassName?: string;
  rightIconClassName?: string;
  labelColorVariant?: "primary" | "notSelected";
  labelCustomColor?: string;
  labelClassName?: string;
}

interface LabelProps {
  title: string;
  className?: string;
  colorVariant?: "primary" | "notSelected";
  customColor?: string;
}

const buttonStyles =
  "group flex h-[22px] cursor-pointer items-center rounded p-1 text-[var(--moss-icon-primary-text)] hover:bg-[var(--moss-icon-primary-background-hover)] disabled:cursor-default disabled:opacity-50";

const labelStyles = cva("overflow-hidden text-md text-ellipsis whitespace-nowrap opacity-100", {
  defaultVariants: {
    variant: "primary",
  },
  variants: {
    variant: {
      primary: "text-[var(--moss-primary-text)]",
      notSelected: "text-[var(--moss-not-selected-item-color)]",
    },
  },
});

const ButtonLabel: React.FC<LabelProps> = ({ title, className, colorVariant = "primary", customColor }) => {
  // Create a unique class name for custom colors
  const customColorClass = customColor ? `text-[${customColor}]` : "";

  return (
    <span
      className={cn(
        labelStyles({
          variant: colorVariant,
        }),
        customColorClass,
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
      className,
      leftIconClassName,
      rightIconClassName,
      labelColorVariant,
      labelCustomColor,
      labelClassName,
      ...props
    },
    ref
  ) => {
    return (
      <button ref={ref} className={cn(buttonStyles, className)} {...props}>
        <div className="flex items-center gap-1">
          <Icon icon={leftIcon} className={cn("size-4", "mr-0.5", leftIconClassName)} />
          <ButtonLabel
            title={title}
            colorVariant={labelColorVariant}
            customColor={labelCustomColor}
            className={labelClassName}
          />
          {rightIcon && <Icon icon={rightIcon} className={cn("size-4", rightIconClassName)} />}
        </div>
      </button>
    );
  }
);

export default IconLabelButton;
