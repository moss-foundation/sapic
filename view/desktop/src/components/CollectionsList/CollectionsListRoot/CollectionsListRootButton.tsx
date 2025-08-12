import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

import { CollectionWithEnvironment } from "../types";

interface CollectionsListRootButtonProps {
  showChildren: boolean;
  setShowChildren: (showChildren: boolean) => void;
  collection: CollectionWithEnvironment;
}

export const CollectionsListRootButton = ({
  showChildren,
  setShowChildren,
  collection,
}: CollectionsListRootButtonProps) => {
  return (
    <div className="z-10 flex items-center gap-2 overflow-hidden">
      <button
        onClick={() => setShowChildren(!showChildren)}
        className="hover:background-(--moss-icon-primary-background-hover) flex h-4 w-4 cursor-pointer items-center justify-center rounded-full"
      >
        <Icon icon="ChevronRight" className={cn(showChildren && "rotate-90")} />
      </button>

      <div className="truncate">{collection.name}</div>
    </div>
  );
};
