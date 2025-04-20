import { IDockviewHeaderActionsProps } from "@repo/moss-tabs";
import { nextId } from "./defaultLayout";
import { Icon } from "@/components";

export const AddPanelButton = (props: IDockviewHeaderActionsProps) => {
  const onClick = () => {
    props.containerApi.addPanel({
      id: `id_${Date.now().toString()}`,
      component: "Default",
      title: `Tab ${nextId()}`,
      position: {
        referenceGroup: props.group,
      },
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
