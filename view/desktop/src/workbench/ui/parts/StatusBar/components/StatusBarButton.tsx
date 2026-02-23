import { ComponentPropsWithoutRef } from "react";

import { Icon, Icons } from "@/lib/ui";
import { cn } from "@/utils";

interface StatusBarButtonProps extends Omit<ComponentPropsWithoutRef<"button">, "id"> {
  icon?: Icons;
  label?: string;
  className?: string;
  iconClassName?: string;
}

export const StatusBarButton = ({ icon, iconClassName, label, className, ...props }: StatusBarButtonProps) => {
  return (
    <button {...props} className={cn("relative flex h-full items-center justify-center", className)}>
      <div className="hover:background-(--moss-secondary-background-hover) text-(--moss-primary-foreground) flex items-center gap-1 rounded py-[3px] pl-1.5 pr-1 transition">
        {icon && <Icon icon={icon} className={cn("size-3.5", iconClassName)} />}
        {label && <span className="">{label}</span>}
      </div>
    </button>
  );
};
