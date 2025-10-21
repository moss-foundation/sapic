import { useState } from "react";

import { MossToggle } from "@/lib/ui/MossToggle";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const ToggleSection = () => {
  const [checked1, setChecked1] = useState(true);
  const [checked2, setChecked2] = useState(false);

  return (
    <KitchenSinkSection header="Toggle">
      <MossToggle checked={checked1} onCheckedChange={setChecked1} />
      <MossToggle checked={checked2} onCheckedChange={setChecked2} />
      <MossToggle checked={true} onCheckedChange={() => {}} disabled={true} labelRight="<-- disabled" />
      <MossToggle checked={false} onCheckedChange={() => {}} disabled={true} labelRight="<-- disabled" />
    </KitchenSinkSection>
  );
};
