import { useState } from "react";

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

  return (
    <div className="group/CollectionEnvironmentsListRoot flex flex-col">
      <div className="group/CollectionEnvironmentsListRootHeader relative flex h-[30px] cursor-pointer items-center justify-between py-2 pr-2 pl-[10px]">
        <CollectionEnvironmentsListRootButton
          showChildren={showChildren}
          setShowChildren={setShowChildren}
          collectionsWithEnvironments={collectionsWithEnvironments}
        />

        <CollectionEnvironmentsListRootActions />

        <CollectionEnvironmentsListItemIndicator />
      </div>

      {showChildren && <CollectionEnvironmentsListChildren collectionsWithEnvironments={collectionsWithEnvironments} />}
    </div>
  );
};
