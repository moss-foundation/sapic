import { HTMLAttributes } from "react";

import { Icon, Icons } from "@/lib/ui";

export const EnvironmentsListItem = ({
  icon,
  label,
  disabled,
  ...props
}: { icon: Icons; label: string; disabled?: boolean } & HTMLAttributes<HTMLButtonElement>) => {
  return (
    <button
      className="hover:background-(--moss-gray-12) flex w-full cursor-pointer items-center gap-2 px-4 py-1 disabled:cursor-not-allowed disabled:opacity-50 disabled:hover:bg-transparent"
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
      <Icon icon={icon} />
      <span>{label}</span>
    </button>
  );
};
