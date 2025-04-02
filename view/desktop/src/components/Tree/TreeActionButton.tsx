import { ComponentPropsWithoutRef } from "react";

import { cn } from "@/utils";

import { Icon, Icons } from "../Icon";

interface TreeActionButtonProps extends ComponentPropsWithoutRef<"button"> {
  icon: Icons;
}

export const TreeActionButton = ({ icon, className, ...props }: TreeActionButtonProps) => {
  return (
    <div className="flex size-[26px] items-center justify-center">
      <button
        className={cn(
          `background-(--moss-icon-primary-background) hover:background-(--moss-icon-primary-background-hover) disabled:hover:background-transparent disabled:hover:dark:background-transparent flex cursor-pointer items-center justify-center rounded-[3px] p-[3px] text-(--moss-icon-primary-text) disabled:cursor-default disabled:opacity-50 disabled:hover:text-(--moss-icon-primary-text)`,
          className
        )}
        {...props}
      >
        <Icon icon={icon} />
      </button>
    </div>
  );
};

export default TreeActionButton;
