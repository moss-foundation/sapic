import React, { useState } from "react";
import { Root as ToggleGroupRoot, Item as ToggleGroupItem } from "@/components/ToggleGroup";
import { cn } from "@/utils";

interface ModeToggleProps {
  defaultValue?: "request" | "design";
  onValueChange?: (value: "request" | "design") => void;
  className?: string;
  compact?: boolean;
}

export const ModeToggle: React.FC<ModeToggleProps> = ({
  defaultValue = "request",
  onValueChange,
  className,
  compact = false,
}) => {
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
      className={cn("rounded-sm border border-[var(--moss-border-color)]", className)}
    >
      <ToggleGroupItem value="request" className="rounded-sm whitespace-nowrap" compact={compact}>
        Request-first mode
      </ToggleGroupItem>
      <ToggleGroupItem value="design" className="rounded-sm whitespace-nowrap" compact={compact}>
        Design-first mode
      </ToggleGroupItem>
    </ToggleGroupRoot>
  );
};

export default ModeToggle;
