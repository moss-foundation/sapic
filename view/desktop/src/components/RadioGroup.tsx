import { useId } from "react";

import { cn } from "@/utils";

import { Icon, RadioGroup as RadioGroupPrimitive } from "../lib/ui";

const radioGroupItemStyles = `
  background-(--moss-radio-bg)  
  border-(--moss-radio-border) 
  
  disabled:background-(--moss-radio-bg-disabled)
  disabled:border-(--moss-radio-border-disabled)
  disabled:data-[state=checked]:background-(--moss-radio-bg-disabled)
  disabled:cursor-not-allowed

  data-[state=checked]:background-(--moss-primary) 

  focus-visible:outline-(--moss-primary) 
`;

export interface ItemWithLabelProps {
  label?: string;
  description?: string;
  value: string;
  checked?: boolean;
  onClick: () => void;
  className?: string;
  disabled?: boolean;
}

const ItemWithLabel = ({
  label,
  description,
  value,
  checked,
  onClick,
  className,
  disabled = false,
}: ItemWithLabelProps) => {
  const id = useId();

  return (
    <div
      className={cn(
        "selection-none grid grid-cols-[min-content_1fr] grid-rows-[repeat(2,min-content)] items-center gap-x-2",
        {
          "cursor-pointer": !disabled,
          "opacity-50": disabled,
        },
        className
      )}
    >
      <RadioGroupPrimitive.Item
        value={value}
        id={id}
        checked={checked}
        onClick={onClick}
        className={cn(radioGroupItemStyles)}
        disabled={disabled}
      >
        <RadioGroupPrimitive.Indicator>
          <Icon icon="RadioIndicator" />
        </RadioGroupPrimitive.Indicator>
      </RadioGroupPrimitive.Item>

      {label && (
        <label
          htmlFor={id}
          className={cn("cursor-pointer py-2", {
            "cursor-not-allowed": disabled,
          })}
        >
          {label}
        </label>
      )}
      {description && (
        <p className="col-start-2 text-left text-xs leading-3.75 text-(--moss-secondary-text)">{description}</p>
      )}
    </div>
  );
};

const Root = RadioGroupPrimitive.Root;

export { ItemWithLabel, Root };
