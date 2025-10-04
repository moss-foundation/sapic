import { useId } from "react";

import { Checkbox as CheckboxPrimitive, Icon } from "@/lib/ui";
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
    <div className={cn("flex shrink-0 items-center gap-2", className)}>
      <CheckboxPrimitive.Root
        id={id}
        className="cursor-pointer rounded-[3px] border-(--moss-checkbox-border)"
        checked={checked}
        onCheckedChange={onCheckedChange}
        disabled={disabled}
      >
        <CheckboxPrimitive.Indicator>
          <Icon icon={checked === "indeterminate" ? "CheckboxIntermediate" : "CheckboxChecked"} />
        </CheckboxPrimitive.Indicator>
      </CheckboxPrimitive.Root>
      {label && (
        <label htmlFor={id} className="cursor-pointer">
          {label}
        </label>
      )}
    </div>
  );
};

export default CheckboxWithLabel;
