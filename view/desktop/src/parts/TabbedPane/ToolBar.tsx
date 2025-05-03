import React from "react";

import { ActionButton, Divider, IconLabelButton } from "@/components";

interface ToolBarProps {
  workspace?: boolean;
}

const ToolBar: React.FC<ToolBarProps> = ({ workspace = false }) => {
  return (
    <div className="group-control mr-0.5 flex h-full items-center px-2 select-none">
      <ActionButton icon="MoreHorizontal" />

      {workspace && (
        <>
          <Divider height="large" className="mr-2.5" />
          <IconLabelButton
            leftIcon="ToolBarEnvironment"
            rightIcon="ChevronDown"
            title="No environment"
            labelClassName="text-[var(--moss-not-selected-item-color)]"
          />
          <ActionButton icon="ToolBarVariables" className="ml-0.5" />
        </>
      )}
    </div>
  );
};

export default ToolBar;
