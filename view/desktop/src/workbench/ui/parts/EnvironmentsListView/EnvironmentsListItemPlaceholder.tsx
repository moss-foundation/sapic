import { HTMLAttributes } from "react";

import { PlannedBadge } from "@/components";
import { Icon, Icons } from "@/lib/ui";
import { cn } from "@/utils";

export const EnvironmentsListItemPlaceholder = ({
  icon,
  label,
  disabled,
  ...props
}: { icon: Icons; label: string; disabled?: boolean } & HTMLAttributes<HTMLButtonElement>) => {
  return (
    <button
      className="hover:background-(--moss-secondary-background-hover) flex w-full cursor-pointer items-center justify-between gap-2 py-1 pl-2.5 pr-2 disabled:cursor-not-allowed disabled:hover:bg-transparent"
      disabled={disabled}
      onClick={(e) => {
        if (disabled) {
          e.preventDefault();
          return;
        }

        props.onClick?.(e);
      }}
      {...props}
    >
      <div className={cn("flex items-center gap-2", { "opacity-50": disabled })}>
        <Icon icon={icon} />
        <span>{label}</span>
      </div>

      <PlannedBadge />
    </button>
  );
};
