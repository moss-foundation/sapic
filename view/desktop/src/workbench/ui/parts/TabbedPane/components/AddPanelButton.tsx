import { IDockviewHeaderActionsProps } from "moss-tabs";

import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { ActionButton } from "@/workbench/ui/components";

import { nextId } from "../DebugComponents/defaultLayout";

export const AddPanelButton = (props: IDockviewHeaderActionsProps) => {
  const { addOrFocusPanel } = useTabbedPaneStore();

  const onClick = () => {
    const tabId = `tab_${Date.now().toString()}`;
    addOrFocusPanel({
      id: tabId,
      title: `Untitled Endpoint ${nextId()}`,
      component: "DefaultView",
      position: {
        referenceGroup: props.group,
      },
    });
  };

  return (
    <div className="group-control flex h-full select-none items-center px-2">
      <ActionButton icon="Add" onClick={onClick} />
    </div>
  );
};
