import { RadioGroup } from "@/workbench/ui/components";

import { Subheader } from "../Sections/Subheader";

interface ModeRadioGroupProps {
  mode: "Default" | "Custom";
  setMode: (mode: "Default" | "Custom") => void;
}

export const ModeRadioGroup = ({ mode, setMode }: ModeRadioGroupProps) => {
  return (
    <div>
      <Subheader>
        <span>Mode</span>
        <div className="background-(--moss-border) my-auto h-px w-full" />
      </Subheader>
      <p className="text-(--moss-secondary-foreground) text-sm leading-5">
        You can switch modes in the workspace at any time and as often as needed.
      </p>
      <div className="pl-5">
        <RadioGroup.Root>
          <RadioGroup.ItemWithLabel
            label="Default"
            description="This mode is suitable when your project is stored in a separate repository or doesn’t have a repository at all."
            value="Default"
            checked={mode === "Default"}
            onClick={() => setMode("Default")}
          />

          <RadioGroup.ItemWithLabel
            label="Custom"
            description="This mode is suitable if you want to store the project in your project’s repository or in any other folder you specify."
            value="Custom"
            checked={mode === "Custom"}
            onClick={() => setMode("Custom")}
            disabled
          />
        </RadioGroup.Root>
      </div>
    </div>
  );
};
