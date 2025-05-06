import { useId } from "react";

import { Checkbox as CheckboxPrimitive, Icon } from "@/lib/ui";
import { CheckedState } from "@radix-ui/react-checkbox";

const CheckboxWithLabel = ({
  checked,
  onCheckedChange,
  label,
}: {
  checked: boolean;
  onCheckedChange: (checked: CheckedState) => void;
  label?: string;
}) => {
  const id = useId();

  return (
    <div className="flex items-center gap-2">
      <CheckboxPrimitive.Root id={id} className="cursor-pointer" checked={checked} onCheckedChange={onCheckedChange}>
        <CheckboxPrimitive.Indicator>
          <Icon icon="CheckboxIndicator" />
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
