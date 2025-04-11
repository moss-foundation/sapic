import { ButtonHTMLAttributes, forwardRef } from "react";

import { Icon, Icons } from "@/components";
import { cn } from "@/utils";

interface TreeActionButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  icon: Icons;
  className?: string;
}

export const TreeActionButton = forwardRef<HTMLButtonElement, TreeActionButtonProps>(
  ({ icon, className, ...props }, ref) => {
    return (
      <button ref={ref} className={cn("flex size-[26px] items-center justify-center", className)} {...props}>
        <div
          className={cn(
            `background-(--moss-icon-primary-background) hover:background-(--moss-icon-primary-background-hover) disabled:hover:background-transparent disabled:hover:dark:background-transparent flex cursor-pointer items-center justify-center rounded-[3px] p-[3px] text-(--moss-icon-primary-text) disabled:cursor-default disabled:opacity-50 disabled:hover:text-(--moss-icon-primary-text)`
          )}
        >
          <Icon icon={icon} />
        </div>
      </button>
    );
  }
);

export default TreeActionButton;
