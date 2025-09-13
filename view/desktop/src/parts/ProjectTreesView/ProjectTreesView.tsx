import { useEffect, useRef, useState } from "react";

import { InputPlain, ProjectTree } from "@/components";
import { useNodeDragAndDropHandler } from "@/components/ProjectTree/hooks/useNodeDragAndDropHandler";
import { useCollectionDragAndDropHandler } from "@/components/ProjectTree/hooks/useProjectDragAndDropHandler";
import { isSourceProjectTreeNode } from "@/components/ProjectTree/utils";
import { useCollectionsTrees } from "@/hooks/collection/derivedHooks/useCollectionsTrees";
import { Scrollbar } from "@/lib/ui";
import { useRequestModeStore } from "@/store/requestMode";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ProjectCreationZone } from "./ProjectCreationZone";
import { CollectionTreeViewHeader } from "./ProjectTreeViewHeader";

export const CollectionTreesView = () => {
  const dropTargetToggleRef = useRef<HTMLDivElement>(null);

  const { displayMode } = useRequestModeStore();

  useCollectionDragAndDropHandler();
  useNodeDragAndDropHandler();

  const [showProjectCreationZone, setShowProjectCreationZone] = useState<boolean>(false);

  useEffect(() => {
    if (!dropTargetToggleRef.current) return;
    const element = dropTargetToggleRef.current;

    return dropTargetForElements({
      element,
      getData: () => ({
        type: "ProjectCreationZone",
      }),
      canDrop({ source }) {
        return isSourceProjectTreeNode(source);
      },
      onDrop() {
        setShowProjectCreationZone(false);
      },
      onDragLeave() {
        setShowProjectCreationZone(false);
      },
      onDragStart() {
        setShowProjectCreationZone(true);
      },
      onDragEnter() {
        setShowProjectCreationZone(true);
      },
    });
  }, []);

  const { collectionsTreesSortedByOrder, isLoading } = useCollectionsTrees();

  return (
    <div ref={dropTargetToggleRef} className="flex h-full flex-col">
      <CollectionTreeViewHeader />

      <Scrollbar className="min-h-0 flex-1" classNames={{ contentEl: "h-full w-full" }}>
        <div className="flex h-full flex-col">
          <div className="flex shrink items-center gap-[7px] px-2 py-1">
            <InputPlain placeholder="Search" />
          </div>

          <div className="flex h-full flex-col">
            {!isLoading &&
              collectionsTreesSortedByOrder.map((collection) => (
                <ProjectTree key={collection.id} tree={collection} displayMode={displayMode} />
              ))}
          </div>

          {showProjectCreationZone && (
            <div className="mt-auto p-2">
              <ProjectCreationZone />
            </div>
          )}
        </div>
      </Scrollbar>
    </div>
  );
};
