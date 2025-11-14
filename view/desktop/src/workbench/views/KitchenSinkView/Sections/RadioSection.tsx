import { useState } from "react";

import { RadioGroup } from "@/workbench/ui/components";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const RadioSection = () => {
  const [radioChecked, setRadioChecked] = useState("radio1");

  return (
    <KitchenSinkSection header="Radio">
      <RadioGroup.Root>
        <RadioGroup.ItemWithLabel
          label="Radio 1"
          description="description"
          value="radio1"
          checked={radioChecked === "radio1"}
          onClick={() => setRadioChecked("radio1")}
        />

        <RadioGroup.ItemWithLabel
          label="Radio 2"
          description="description"
          value="radio2"
          checked={radioChecked === "radio2"}
          onClick={() => setRadioChecked("radio2")}
        />

        <RadioGroup.ItemWithLabel
          label="Radio 3"
          description="description"
          value="radio3"
          disabled={true}
          checked={true}
          onClick={() => {}}
        />
        <RadioGroup.ItemWithLabel
          label="Radio 4"
          description="description"
          value="radio4"
          disabled={true}
          checked={false}
          onClick={() => {}}
        />
      </RadioGroup.Root>
    </KitchenSinkSection>
  );
};
