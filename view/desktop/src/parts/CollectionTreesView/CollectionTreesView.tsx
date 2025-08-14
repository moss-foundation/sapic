import { useEffect, useRef, useState } from "react";

import { CollectionTree, InputPlain } from "@/components";
import { useCollectionDragAndDropHandler } from "@/components/CollectionTree/hooks/useCollectionDragAndDropHandler";
import { useNodeDragAndDropHandler } from "@/components/CollectionTree/hooks/useNodeDragAndDropHandler";
import { isSourceTreeCollectionNode } from "@/components/CollectionTree/utils";
import { useCollectionsTrees } from "@/hooks/collection/derivedHooks/useCollectionsTrees";
import { Scrollbar } from "@/lib/ui";
import { useRequestModeStore } from "@/store/requestMode";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { CollectionCreationZone } from "./CollectionCreationZone";
import { CollectionTreeViewHeader } from "./CollectionTreeViewHeader";

export const CollectionTreesView = () => {
  const dropTargetToggleRef = useRef<HTMLDivElement>(null);

  const { displayMode } = useRequestModeStore();

  useCollectionDragAndDropHandler();
  useNodeDragAndDropHandler();

  const [showCollectionCreationZone, setShowCollectionCreationZone] = useState<boolean>(false);

  useEffect(() => {
    if (!dropTargetToggleRef.current) return;
    const element = dropTargetToggleRef.current;

    return dropTargetForElements({
      element,
      getData: () => ({
        type: "CollectionCreationZone",
      }),
      canDrop({ source }) {
        return isSourceTreeCollectionNode(source);
      },
      onDrop() {
        setShowCollectionCreationZone(false);
      },
      onDragLeave() {
        setShowCollectionCreationZone(false);
      },
      onDragStart() {
        setShowCollectionCreationZone(true);
      },
      onDragEnter() {
        setShowCollectionCreationZone(true);
      },
    });
  }, []);

  const { collectionsTrees, isLoading } = useCollectionsTrees();

  return (
    <div className="flex h-full flex-col">
      <CollectionTreeViewHeader />

      <div ref={dropTargetToggleRef} className="relative h-full select-none">
        <Scrollbar className="h-full">
          <div className="flex h-full flex-col">
            <div className="flex shrink items-center gap-[7px] px-2 py-1">
              <InputPlain placeholder="Search" size="sm" />
            </div>

            <div className="flex grow flex-col">
              {!isLoading &&
                collectionsTrees
                  .sort((a, b) => a.order! - b.order!)
                  .map((collection) => (
                    <CollectionTree key={collection.id} tree={collection} displayMode={displayMode} />
                  ))}
            </div>

            {showCollectionCreationZone && (
              <div className="flex justify-end p-2">
                <CollectionCreationZone />
              </div>
            )}
          </div>
        </Scrollbar>
      </div>
    </div>
  );
};
