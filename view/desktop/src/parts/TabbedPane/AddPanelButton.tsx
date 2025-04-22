import { IDockviewHeaderActionsProps } from "@repo/moss-tabs";
import { nextId } from "./defaultLayout";
import { Icon } from "@/components";
import { useTabbedPaneStore } from "@/store/tabbedPane";

export const AddPanelButton = (props: IDockviewHeaderActionsProps) => {
  const { addOrFocusPanel } = useTabbedPaneStore();

  const onClick = () => {
    const tabId = `tab_${Date.now().toString()}`;
    addOrFocusPanel({
      id: tabId,
      component: "Default",
      params: {
        iconType: "file",
      },
      position: {
        referenceGroup: props.group,
      },
      title: `Untitled Request ${nextId()}`,
    });
  };

  return (
    <div className="group-control flex h-full items-center px-2 select-none">
      <div className="cursor-pointer rounded p-1 hover:bg-[var(--moss-icon-primary-background-hover)]">
        <Icon onClick={onClick} icon="Plus" className="text-[var(--moss-icon-primary-text)]" />
      </div>
    </div>
  );
};
