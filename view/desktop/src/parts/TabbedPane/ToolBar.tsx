import React, { ButtonHTMLAttributes, forwardRef } from "react";
import { ActionButton, Divider, Icon, type Icons } from "@/components";
import { cn } from "@/utils";

interface ToolBarButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  leftIcon: Icons;
  rightIcon: Icons;
  title: string;
  className?: string;
}

interface ToolBarTitleProps {
  title: string;
}

const ToolBarTitle: React.FC<ToolBarTitleProps> = ({ title }) => {
  return (
    <span className="overflow-hidden text-xs text-ellipsis whitespace-nowrap text-[var(--moss-not-selected-item-color)] opacity-100">
      {title}
    </span>
  );
};

const ToolBarButton = forwardRef<HTMLButtonElement, ToolBarButtonProps>(
  ({ leftIcon, rightIcon, title, className, ...props }, ref) => {
    return (
      <button
        ref={ref}
        className={cn(
          "group flex h-[22px] cursor-pointer items-center rounded p-1 text-[var(--moss-icon-primary-text)]",
          "hover:bg-[var(--moss-icon-primary-background-hover)]",
          "disabled:cursor-default disabled:opacity-50",
          className
        )}
        {...props}
      >
        <div className="flex items-center gap-1">
          <Icon icon={leftIcon} className="mr-0.5" />
          <ToolBarTitle title={title} />
          <Icon icon={rightIcon} />
        </div>
      </button>
    );
  }
);

interface ToolBarProps {
  workspace?: boolean;
}

const ToolBar: React.FC<ToolBarProps> = ({ workspace = false }) => {
  return (
    <div className="group-control mr-[4px] flex h-full items-center px-2 select-none">
      <ActionButton icon="ThreeVerticalDots" />

      {workspace && (
        <>
          <Divider height="large" className="mr-2.5" />
          <ToolBarButton leftIcon="ToolBarEnvironment" rightIcon="ChevronDown" title="No environment" />
          <ActionButton icon="ToolBarVariables" className="ml-0.5" />
        </>
      )}
    </div>
  );
};

export default ToolBar;
