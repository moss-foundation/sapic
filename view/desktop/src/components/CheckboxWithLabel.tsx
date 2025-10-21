import { useId } from "react";

import { Checkbox as CheckboxPrimitive } from "@/lib/ui";
import { cn } from "@/utils";
import { CheckedState } from "@radix-ui/react-checkbox";

interface CheckboxWithLabelProps {
  checked: CheckedState;
  onCheckedChange: (checked: CheckedState) => void;
  label?: string;
  disabled?: boolean;
  className?: string;
}

const CheckboxWithLabel = ({ checked, onCheckedChange, label, disabled, className }: CheckboxWithLabelProps) => {
  const id = useId();

  return (
    <div
      className={cn(
        "flex shrink-0 items-center gap-2",
        {
          "text-(--moss-foreground-disabled)": disabled,
        },
        className
      )}
    >
      <CheckboxPrimitive.Root
        id={id}
        className="cursor-pointer rounded-[3px] border-(--moss-border) disabled:border-(--moss-border-disabled)"
        checked={checked}
        onCheckedChange={onCheckedChange}
        disabled={disabled}
      >
        <CheckboxPrimitive.Indicator>
          {checked === "indeterminate" ? <IntermediateSVG /> : <CheckedSVG />}
        </CheckboxPrimitive.Indicator>
      </CheckboxPrimitive.Root>
      {label && (
        <label htmlFor={id} className={cn("cursor-pointer", { "cursor-not-allowed": disabled })}>
          {label}
        </label>
      )}
    </div>
  );
};

const CheckedSVG = () => {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="16" height="16" rx="3" fill="var(--moss-accent)" />
      <path d="M4 8.5L7 11.5L12.5 5" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
};

const IntermediateSVG = () => {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect width="16" height="16" rx="3" fill="var(--moss-accent)" />
      <line x1="3" y1="8" x2="13" y2="8" stroke="white" strokeWidth="2" strokeLinecap="round" />
    </svg>
  );
};

export default CheckboxWithLabel;
