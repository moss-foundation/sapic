import { ActionButton } from "@/components";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { IDockviewHeaderActionsProps } from "@repo/moss-tabs";

import { nextId } from "./defaultLayout";

export const AddPanelButton = (props: IDockviewHeaderActionsProps) => {
  const { addOrFocusPanel } = useTabbedPaneStore();

  const onClick = () => {
    const tabId = `tab_${Date.now().toString()}`;
    addOrFocusPanel({
      id: tabId,
      component: "Default",
      params: {
        workspace: true,
      },
      position: {
        referenceGroup: props.group,
      },
      title: `Untitled Endpoint ${nextId()}`,
    });
  };

  return (
    <div className="group-control flex h-full items-center px-2 select-none">
      <ActionButton icon="Add" onClick={onClick} />
    </div>
  );
};
