import { useState } from "react";

import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { CollectionEnvironmentsListItemIndicator } from "../CollectionEnvironmentsListItemIndicator";
import { CollectionEnvironmentsListItemActions } from "./CollectionEnvironmentsListItemActions";
import { CollectionEnvironmentsListItemButton } from "./CollectionEnvironmentsListItemButton";

interface CollectionEnvironmentsListItemProps {
  environment: StreamEnvironmentsEvent;
}

export const CollectionEnvironmentsListItem = ({ environment }: CollectionEnvironmentsListItemProps) => {
  const [showActionMenu, setShowActionMenu] = useState(false);
  const { activePanelId, addOrFocusPanel } = useTabbedPaneStore();

  const onClick = () => {
    addOrFocusPanel({
      id: `CollectionEnvironmentsListItem-${environment.id}`,
      component: "Default",
      title: environment.name,
      params: {
        iconType: "CollectionEnvironment",
      },
    });
  };

  const isActive = activePanelId === `CollectionEnvironmentsListItem-${environment.id}`;

  return (
    <div
      className="group/CollectionEnvironmentsListItem relative flex h-[26px] cursor-pointer items-center justify-between gap-1 pr-2 pl-5.5"
      onClick={onClick}
      role="button"
      tabIndex={0}
    >
      <CollectionEnvironmentsListItemButton label={environment.name} />

      <CollectionEnvironmentsListItemActions showActionMenu={showActionMenu} setShowActionMenu={setShowActionMenu} />

      <CollectionEnvironmentsListItemIndicator isActive={isActive} />
    </div>
  );
};
