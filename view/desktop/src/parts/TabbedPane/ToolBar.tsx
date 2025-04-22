import React from "react";
import { Divider, Icon, type Icons } from "@/components";

interface ToolBarButtonProps {
  leftIcon: Icons;
  rightIcon: Icons;
  title: string;
  className?: string;
}

const ToolBarButton: React.FC<ToolBarButtonProps> = ({ leftIcon, rightIcon, title, className }) => {
  return (
    <div
      className={`group flex h-[24px] cursor-pointer items-center rounded p-1 hover:bg-[var(--moss-icon-primary-background-hover)] ${className || ""}`}
    >
      <div className="flex items-center gap-1">
        <Icon
          icon={leftIcon}
          className="mr-[2px] size-[18px] text-[var(--moss-icon-primary-text)] group-hover:text-black"
        />
        <span className="text-xs text-[var(--moss-icon-primary-text)] opacity-70 group-hover:text-black group-hover:opacity-100">
          {title}
        </span>
        <Icon
          icon={rightIcon}
          className="text-[var(--moss-icon-primary-text)] opacity-70 group-hover:text-black group-hover:opacity-100"
        />
      </div>
    </div>
  );
};

interface ToolBarProps {
  workspace?: boolean;
}

const ToolBar: React.FC<ToolBarProps> = ({ workspace = false }) => {
  return (
    <div className="group-control mr-[10px] flex h-full items-center px-2 select-none">
      <div className="group flex h-[24px] cursor-pointer items-center rounded p-1 hover:bg-[var(--moss-icon-primary-background-hover)]">
        <Icon icon="ThreeVerticalDots" className="text-[var(--moss-icon-primary-text)] group-hover:text-black" />
      </div>

      {workspace && (
        <>
          <Divider height="large" className="mr-[10px]" />
          <ToolBarButton leftIcon="ToolBarEnvironment" rightIcon="ChevronDown" title="No environment" />
          <div className="group ml-[3px] flex h-[24px] cursor-pointer items-center rounded p-1 hover:bg-[var(--moss-icon-primary-background-hover)]">
            <Icon icon="ToolBarVariables" className="text-[var(--moss-icon-primary-text)] group-hover:text-black" />
          </div>
        </>
      )}
    </div>
  );
};

export default ToolBar;
