import { FormEvent, useState } from "react";

import Input from "@/lib/ui/Input";
import { ToggleButton } from "@/lib/ui/ToggleButton";
import { RadioGroup } from "@/workbench/ui/components";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const InputsSection = () => {
  const [contrast, setContrast] = useState(false);
  const [intent, setIntent] = useState<"plain" | "outlined">("plain");

  const handleFormSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
  };

  return (
    <KitchenSinkSection header="Inputs">
      <div className="controls flex w-fit flex-col gap-2">
        <ToggleButton checked={contrast} onCheckedChange={setContrast} labelRight="Contrast" />
        <RadioGroup.Root value={intent} onValueChange={(value) => setIntent(value as "plain" | "outlined")}>
          <RadioGroup.ItemWithLabel
            label="Outlined"
            value="outlined"
            checked={intent === "outlined"}
            onClick={() => setIntent("outlined")}
          />
          <RadioGroup.ItemWithLabel
            label="Plain"
            value="plain"
            checked={intent === "plain"}
            onClick={() => setIntent("plain")}
          />
        </RadioGroup.Root>
      </div>

      <div className="flex flex-col gap-2">
        <Input placeholder="Input Sapic" contrast={contrast} intent={intent} />
        <Input placeholder="Input Sapic" contrast={contrast} intent={intent} iconLeft="Add" shortcut="⌘+S" />

        <form onSubmit={handleFormSubmit}>
          <Input
            placeholder="Invalid input"
            data-invalid={true}
            required
            contrast={contrast}
            intent={intent}
            iconLeft="Add"
            shortcut="⌘+S"
          />
        </form>
      </div>
    </KitchenSinkSection>
  );
};
