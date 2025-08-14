import { useState } from "react";

import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { GroupedEnvironmentsListItemIndicator } from "../GroupedEnvironmentsListItemIndicator";
import { GroupedEnvironmentsListItemActions } from "./GroupedEnvironmentsListItemActions";
import { GroupedEnvironmentsListItemButton } from "./GroupedEnvironmentsListItemButton";

interface GroupedEnvironmentsListItemProps {
  environment: StreamEnvironmentsEvent;
}

export const GroupedEnvironmentsListItem = ({ environment }: GroupedEnvironmentsListItemProps) => {
  const [showActionMenu, setShowActionMenu] = useState(false);
  const { activePanelId, addOrFocusPanel } = useTabbedPaneStore();

  const onClick = () => {
    addOrFocusPanel({
      id: `GroupedEnvironmentsListItem-${environment.id}`,
      component: "Default",
      title: environment.name,
      params: {
        iconType: "GroupedEnvironment",
      },
    });
  };

  const isActive = activePanelId === `GroupedEnvironmentsListItem-${environment.id}`;

  return (
    <div
      className="group/GroupedEnvironmentsListItem relative flex h-[26px] cursor-pointer items-center justify-between gap-1 pr-2 pl-5.5"
      onClick={onClick}
      role="button"
      tabIndex={0}
    >
      <GroupedEnvironmentsListItemButton label={environment.name} />

      <GroupedEnvironmentsListItemActions showActionMenu={showActionMenu} setShowActionMenu={setShowActionMenu} />

      <GroupedEnvironmentsListItemIndicator isActive={isActive} />
    </div>
  );
};
