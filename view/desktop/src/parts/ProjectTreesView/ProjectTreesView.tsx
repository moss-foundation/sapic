import { useEffect, useRef, useState } from "react";

import { ProjectTree } from "@/components";
import { useNodeDragAndDropHandler } from "@/components/ProjectTree/hooks/useNodeDragAndDropHandler";
import { useProjectDragAndDropHandler } from "@/components/ProjectTree/hooks/useProjectDragAndDropHandler";
import { isSourceProjectTreeNode } from "@/components/ProjectTree/utils";
import { useProjectsTrees } from "@/hooks/project/derivedHooks/useProjectsTrees";
import { Scrollbar } from "@/lib/ui";
import Input from "@/lib/ui/Input";
import { useWorkspaceModeStore } from "@/store/workspaceMode";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ProjectCreationZone } from "./ProjectCreationZone";
import { ProjectTreeViewHeader } from "./ProjectTreeViewHeader";

export const ProjectTreesView = () => {
  const dropTargetToggleRef = useRef<HTMLDivElement>(null);

  const { displayMode } = useWorkspaceModeStore();

  useProjectDragAndDropHandler();
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

  const { projectsTreesSortedByOrder, isLoading } = useProjectsTrees();

  return (
    <div ref={dropTargetToggleRef} className="flex h-full flex-col">
      <ProjectTreeViewHeader />

      <Scrollbar className="min-h-0 flex-1" classNames={{ contentEl: "h-full w-full" }}>
        <div className="flex h-full flex-col">
          <div className="flex shrink items-center gap-[7px] px-2 py-1">
            <Input placeholder="Search" />
          </div>

          <div className="flex h-full flex-col">
            {!isLoading &&
              projectsTreesSortedByOrder.map((p) => <ProjectTree key={p.id} tree={p} displayMode={displayMode} />)}
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
