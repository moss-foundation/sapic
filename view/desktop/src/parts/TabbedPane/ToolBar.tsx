import React, { ButtonHTMLAttributes } from "react";
import { ActionButton, Divider, Icon, IconLabelButton, type Icons } from "@/components";
import { cn } from "@/utils";

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

interface ToolBarProps {
  workspace?: boolean;
}

const ToolBar: React.FC<ToolBarProps> = ({ workspace = false }) => {
  return (
    <div className="group-control mr-0.5 flex h-full items-center px-2 select-none">
      <ActionButton icon="ThreeVerticalDots" />

      {workspace && (
        <>
          <Divider height="large" className="mr-2.5" />
          <IconLabelButton leftIcon="ToolBarEnvironment" rightIcon="ChevronDown" title="No environment" />
          <ActionButton icon="ToolBarVariables" className="ml-0.5" />
        </>
      )}
    </div>
  );
};

export default ToolBar;
