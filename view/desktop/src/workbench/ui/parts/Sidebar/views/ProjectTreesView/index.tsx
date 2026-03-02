import { useEffect, useRef, useState } from "react";

import { useSyncEnvironments } from "@/db/environmentsSummaries/hooks/useSyncEnvironments";
import { Scrollbar } from "@/lib/ui";
import Input from "@/lib/ui/Input";
import { WorkspaceEnvironmentsList } from "@/workbench/ui/components";
import { useMonitorProjectRootNodes } from "@/workbench/ui/components/ProjectTree/dnd/hooks/useMonitorProjectRootNodes";
import { useNodeDragAndDropHandler } from "@/workbench/ui/components/ProjectTree/TreeNode/dnd/hooks/useNodeDragAndDropHandler";
import { isSourceProjectTreeNode } from "@/workbench/ui/components/ProjectTree/utils";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ProjectCreationZone } from "./ProjectCreationZone";
import { ProjectTreesList } from "./ProjectTreesList";
import { ProjectTreeViewHeader } from "./ProjectTreeViewHeader";

export const ProjectTreesView = () => {
  const dropTargetToggleRef = useRef<HTMLDivElement>(null);

  useMonitorProjectRootNodes();
  useNodeDragAndDropHandler();

  useSyncEnvironments();

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

  return (
    <div ref={dropTargetToggleRef} className="flex h-full flex-col">
      <ProjectTreeViewHeader />

      <Scrollbar className="min-h-0 flex-1" classNames={{ contentEl: "h-full w-full" }}>
        <div className="flex h-full flex-col">
          <div className="flex shrink items-center gap-[7px] px-2 py-1">
            <Input intent="outlined" contrast={true} placeholder="Search" shortcut="⌘+S" />
          </div>

          <WorkspaceEnvironmentsList />

          <ProjectTreesList />

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
