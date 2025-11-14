import { useState } from "react";

import { ToggleButton } from "@/lib/ui/ToggleButton";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const ToggleSection = () => {
  const [checked1, setChecked1] = useState(true);
  const [checked2, setChecked2] = useState(false);

  return (
    <KitchenSinkSection header="Toggle">
      <ToggleButton checked={checked1} onCheckedChange={setChecked1} />
      <ToggleButton checked={checked2} onCheckedChange={setChecked2} />
      <ToggleButton checked={true} onCheckedChange={() => {}} disabled={true} labelRight="<-- disabled" />
      <ToggleButton checked={false} onCheckedChange={() => {}} disabled={true} labelRight="<-- disabled" />
    </KitchenSinkSection>
  );
};
