import { useState } from "react";

import CheckboxWithLabel from "@/lib/ui/CheckboxWithLabel";
import { CheckedState } from "@radix-ui/react-checkbox";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const CheckboxSection = () => {
  const [checkboxes, setCheckboxes] = useState<
    { id: string; checked: CheckedState; label: string; disabled: boolean }[]
  >([
    {
      id: "1",
      checked: true,
      label: "Checkbox 1",
      disabled: false,
    },
    {
      id: "2",
      checked: true,
      label: "Checkbox",
      disabled: false,
    },
    {
      id: "3",
      checked: false,
      label: "Checkbox Disabled",
      disabled: true,
    },
  ]);

  const handleAllCheckboxesChange = (checked: CheckedState) => {
    setCheckboxes(
      checkboxes.map((checkbox) => {
        if (checkbox.disabled) return checkbox;
        return { ...checkbox, checked };
      })
    );
  };

  const enabledCheckboxes = checkboxes.filter((checkbox) => !checkbox.disabled);
  const allCheckboxesChecked = enabledCheckboxes.every((checkbox) => checkbox.checked);
  const someCheckboxesChecked = enabledCheckboxes.some((checkbox) => checkbox.checked);

  const headerCheckedState = allCheckboxesChecked ? true : someCheckboxesChecked ? "indeterminate" : false;

  return (
    <KitchenSinkSection header="Checkbox">
      <CheckboxWithLabel
        checked={headerCheckedState}
        onCheckedChange={handleAllCheckboxesChange}
        label="All checkboxes"
      />
      <hr />

      {checkboxes.map((checkbox) => (
        <CheckboxWithLabel
          key={checkbox.id}
          checked={checkbox.checked}
          onCheckedChange={(checked) =>
            setCheckboxes(checkboxes.map((c) => (c.id === checkbox.id ? { ...c, checked } : c)))
          }
          label={checkbox.label}
          disabled={checkbox.disabled}
        />
      ))}
    </KitchenSinkSection>
  );
};
