import { ReactNode } from "react";

import { PillTabs } from "@/lib/ui/Tabs/index";
import { cn } from "@/utils";

import { ProviderIcon, ProviderIconType } from "./ProviderIcon";

export type ProviderType = "github" | "gitlab";

interface VcsProviderSwitcherProps {
  value: string;
  onValueChange: (value: string) => void;
  label?: string;
  disabled?: boolean;
  layout?: "vertical" | "grid";
  children?: ReactNode;
  className?: string;
}

export const VcsProviderSwitcher = ({
  value,
  onValueChange,
  label = "Provider:",
  disabled = false,
  layout = "vertical",
  children,
  className,
}: VcsProviderSwitcherProps) => {
  // Normalize value to lowercase for icon matching
  const normalizedValue = value.toLowerCase() as ProviderIconType;

  if (layout === "grid") {
    return (
      <PillTabs.Root
        value={normalizedValue}
        onValueChange={(val) => onValueChange(val)}
        className={cn("contents", className)}
      >
        <div className={cn(disabled && "opacity-50")}>{label}</div>
        <PillTabs.List className="col-span-2 grid h-min grid-cols-subgrid grid-rows-subgrid">
          <div className="flex gap-2">
            <PillTabs.Trigger
              value="github"
              label="GitHub"
              leadingContent={<ProviderIcon icon="github" />}
              disabled={disabled}
            />
            <PillTabs.Trigger
              value="gitlab"
              label="GitLab"
              leadingContent={<ProviderIcon icon="gitlab" />}
              disabled={disabled}
            />
          </div>
        </PillTabs.List>

        {children}
      </PillTabs.Root>
    );
  }

  // Vertical layout (default)
  return (
    <PillTabs.Root
      value={normalizedValue}
      onValueChange={(val) => onValueChange(val)}
      className={cn("flex flex-col gap-2.5", className)}
    >
      <div className="flex items-center gap-3">
        <span>{label}</span>
        <PillTabs.List>
          <div className="flex gap-2">
            <PillTabs.Trigger
              value="github"
              label="GitHub"
              leadingContent={<ProviderIcon icon="github" />}
              disabled={disabled}
            />
            <PillTabs.Trigger
              value="gitlab"
              label="GitLab"
              leadingContent={<ProviderIcon icon="gitlab" />}
              disabled={disabled}
            />
          </div>
        </PillTabs.List>
      </div>

      {children}
    </PillTabs.Root>
  );
};
