import React, { useState } from "react";
import { Root as ToggleGroupRoot, Item as ToggleGroupItem } from "@/components/ToggleGroup";
import { cn } from "@/utils";

interface ModeToggleProps {
  defaultValue?: "request" | "design";
  onValueChange?: (value: "request" | "design") => void;
  className?: string;
}

export const ModeToggle: React.FC<ModeToggleProps> = ({ defaultValue = "request", onValueChange, className }) => {
  const [value, setValue] = useState<"request" | "design">(defaultValue);

  const handleValueChange = (newValue: string) => {
    if (newValue === "request" || newValue === "design") {
      setValue(newValue);
      onValueChange?.(newValue);
    }
  };

  return (
    <ToggleGroupRoot
      type="single"
      value={value}
      onValueChange={handleValueChange}
      className={cn("rounded-lg border border-[var(--moss-border-color)]", className)}
    >
      <ToggleGroupItem value="request" className="rounded-l-lg whitespace-nowrap">
        Request-first mode
      </ToggleGroupItem>
      <ToggleGroupItem value="design" className="rounded-r-lg whitespace-nowrap">
        Design-first mode
      </ToggleGroupItem>
    </ToggleGroupRoot>
  );
};

export default ModeToggle;
