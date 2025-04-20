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
    <div className="group-control flex h-full items-center px-2 text-[var(--moss-activegroup-visiblepanel-tab-color)] select-none">
      <Icon onClick={onClick} icon="Plus" className="text-(--moss-icon-primary-text)" />
    </div>
  );
};
