import { useState } from "react";

import { useTabbedPaneStore } from "@/store/tabbedPane";

import { CollectionEnvironmentsListChildren } from "../CollectionEnvironmentsListChildren";
import { CollectionEnvironmentsListItemIndicator } from "../CollectionEnvironmentsListItemIndicator";
import { CollectionWithEnvironment } from "../types";
import { CollectionEnvironmentsListRootActions } from "./CollectionEnvironmentsListRootActions";
import { CollectionEnvironmentsListRootButton } from "./CollectionEnvironmentsListRootButton";

interface CollectionEnvironmentsListRootProps {
  collectionsWithEnvironments: CollectionWithEnvironment;
}

export const CollectionEnvironmentsListRoot = ({
  collectionsWithEnvironments,
}: CollectionEnvironmentsListRootProps) => {
  const [showChildren, setShowChildren] = useState(true);
  const { activePanelId, addOrFocusPanel } = useTabbedPaneStore();

  const isActive = activePanelId === collectionsWithEnvironments.id;

  const onClick = () => {
    addOrFocusPanel({
      id: collectionsWithEnvironments.id,
      component: "Default",
      title: collectionsWithEnvironments.name,
      params: {
        iconType: "Collection",
      },
    });
  };

  return (
    <div className="group/CollectionEnvironmentsListRoot flex flex-col">
      <div
        className="group/CollectionEnvironmentsListRootHeader relative flex h-[30px] cursor-pointer items-center justify-between py-2 pr-2 pl-[10px]"
        onClick={onClick}
        role="button"
        tabIndex={0}
      >
        <CollectionEnvironmentsListRootButton
          showChildren={showChildren}
          setShowChildren={setShowChildren}
          collectionsWithEnvironments={collectionsWithEnvironments}
        />

        <CollectionEnvironmentsListRootActions />

        <CollectionEnvironmentsListItemIndicator isActive={isActive} />
      </div>

      {showChildren && <CollectionEnvironmentsListChildren collectionsWithEnvironments={collectionsWithEnvironments} />}
    </div>
  );
};
