import { IDockviewHeaderActionsProps } from "moss-tabs";

import { ActionButton, Divider, IconLabelButton } from "@/components";

interface TabbedPaneToolBarProps extends IDockviewHeaderActionsProps {
  workspace?: boolean;
}

export const TabbedPaneToolBar = ({ workspace = false }: TabbedPaneToolBarProps) => {
  return (
    <div className="group-control mr-0.5 flex h-full select-none items-center px-2">
      <ActionButton icon="MoreHorizontal" />

      {workspace && (
        <>
          <Divider className="py-2" />
          <IconLabelButton leftIcon="Env" rightIcon="ChevronDown" title="No environment" />
          <ActionButton icon="ToolBarVariables" className="ml-0.5" />
        </>
      )}
    </div>
  );
};
