import { ReactNode, useId } from "react";

import { cn } from "@/utils";

import { RadioGroup as RadioGroupPrimitive } from "../../../lib/ui";
import SelectOutlined from "./SelectOutlined";

const radioGroupItemStyles = `
  background-(--moss-controls-background)  
  border-(--moss-controls-border) 
  
  disabled:background-(--moss-background-disabled)
  disabled:border-(--moss-border-disabled)
  disabled:data-[state=checked]:background-(--moss-accent)
  disabled:cursor-not-allowed

  data-[state=checked]:background-(--moss-accent) 

  focus-visible:outline-(--moss-accent) 
  focus-visible:outline-offset-2
  focus-visible:outline-2
`;

export interface ItemWithLabelProps {
  label?: string;
  description?: string | ReactNode;
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
          <RadioIndicatorSVG />
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
        <div className="text-(--moss-secondary-foreground) col-start-2 text-left text-sm leading-4">{description}</div>
      )}
    </div>
  );
};

export interface ItemWithSelectProps {
  label?: string;
  description?: string;
  value: string;
  checked?: boolean;
  onClick: () => void;
  className?: string;
  disabled?: boolean;
  options?: { label: string; value: string }[];
  onChange: (value: string) => void;
  placeholder?: string;
  selectValue?: string;
  required?: boolean;
}
const ItemWithSelect = ({
  label,
  description,
  value,
  checked,
  onClick,
  className,
  disabled = false,
  options,
  onChange,
  placeholder,
  selectValue,
  required = false,
}: ItemWithSelectProps) => {
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
        required={required}
      >
        <RadioGroupPrimitive.Indicator>
          <RadioIndicatorSVG />
        </RadioGroupPrimitive.Indicator>
      </RadioGroupPrimitive.Item>

      <div className="flex items-center gap-2">
        <label
          htmlFor={id}
          className={cn("cursor-pointer py-2", {
            "cursor-not-allowed": disabled,
          })}
        >
          {label}
        </label>

        {/* This wrapper is needed to align the validation message from default select if required attribute is used */}
        <div className="flex items-end justify-center">
          <SelectOutlined.Root disabled={disabled} value={selectValue} onValueChange={onChange} required={required}>
            <SelectOutlined.Trigger placeholder={placeholder} disabled={disabled} />

            <SelectOutlined.Content align="end">
              {options?.map((option) => (
                <SelectOutlined.Item key={option.value} value={option.value} disabled={disabled}>
                  {option.label}
                </SelectOutlined.Item>
              ))}
            </SelectOutlined.Content>
          </SelectOutlined.Root>
        </div>
      </div>

      {description && (
        <div className="text-(--moss-secondary-foreground) col-start-2 text-left text-sm leading-4">{description}</div>
      )}
    </div>
  );
};

const RadioIndicatorSVG = () => {
  return (
    <svg
      width="16"
      height="16"
      viewBox="0 0 16 16"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className="text-white dark:text-black"
    >
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M7.99967 10.6673C9.47247 10.6673 10.6663 9.47345 10.6663 8.00065C10.6663 6.5279 9.47247 5.33398 7.99967 5.33398C6.52692 5.33398 5.33301 6.5279 5.33301 8.00065C5.33301 9.47345 6.52692 10.6673 7.99967 10.6673Z"
        fill="currentColor"
      />
    </svg>
  );
};

const Root = RadioGroupPrimitive.Root;

export { ItemWithLabel, ItemWithSelect, Root };
