import { useState } from "react";

import { useTabbedPaneStore } from "@/store/tabbedPane";

import { CollectionsListChildren } from "../CollectionsListChildren";
import { CollectionsListItemIndicator } from "../CollectionsListItemIndicator";
import { CollectionWithEnvironment } from "../types";
import { CollectionsListRootActions } from "./CollectionsListRootActions";
import { CollectionsListRootButton } from "./CollectionsListRootButton";

interface CollectionsListRootProps {
  collection: CollectionWithEnvironment;
}

export const CollectionsListRoot = ({ collection }: CollectionsListRootProps) => {
  const [showChildren, setShowChildren] = useState(true);
  const { activePanelId, addOrFocusPanel } = useTabbedPaneStore();

  const isActive = activePanelId === collection.id;

  const onClick = () => {
    addOrFocusPanel({
      id: collection.id,
      component: "Default",
      title: collection.name,
      params: {
        iconType: "Collection",
      },
    });
  };

  return (
    <div className="group/CollectionsListRoot flex flex-col">
      <div
        className="group/CollectionsListRootHeader relative flex h-[30px] cursor-pointer items-center justify-between p-2"
        onClick={onClick}
        role="button"
        tabIndex={0}
      >
        <CollectionsListRootButton
          showChildren={showChildren}
          setShowChildren={setShowChildren}
          collection={collection}
        />

        <CollectionsListRootActions />

        <CollectionsListItemIndicator isActive={isActive} />
      </div>

      {showChildren && <CollectionsListChildren collection={collection} />}
    </div>
  );
};
