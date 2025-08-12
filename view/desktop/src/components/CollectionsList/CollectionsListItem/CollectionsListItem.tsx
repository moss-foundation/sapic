import { useState } from "react";

import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { CollectionsListItemIndicator } from "../CollectionsListItemIndicator";
import { CollectionsListItemActions } from "./CollectionsListItemActions";
import { CollectionsListItemButton } from "./CollectionsListItemButton";

interface CollectionsListItemProps {
  environment: StreamEnvironmentsEvent;
}

export const CollectionsListItem = ({ environment }: CollectionsListItemProps) => {
  const [showActionMenu, setShowActionMenu] = useState(false);
  const { activePanelId, addOrFocusPanel } = useTabbedPaneStore();

  const onClick = () => {
    addOrFocusPanel({
      id: `CollectionsListItem-${environment.id}`,
      component: "Default",
      title: environment.name,
      params: {
        iconType: "CollectionEnvironment",
      },
    });
  };

  const isActive = activePanelId === `CollectionsListItem-${environment.id}`;

  return (
    <div
      className="group/CollectionsListItem relative flex h-[26px] cursor-pointer items-center justify-between gap-1 pr-2 pl-5.5"
      onClick={onClick}
      role="button"
      tabIndex={0}
    >
      <CollectionsListItemButton label={environment.name} />

      <CollectionsListItemActions showActionMenu={showActionMenu} setShowActionMenu={setShowActionMenu} />

      <CollectionsListItemIndicator isActive={isActive} />
    </div>
  );
};
