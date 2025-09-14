import { useCallback, useEffect } from "react";

import { useProjectsTrees } from "@/hooks";
import { useBatchUpdateProject } from "@/hooks/project/useBatchUpdateProject";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ProjectDragType } from "../constants";
import { getTreeRootNodeSourceData, getTreeRootNodeTargetData } from "../utils";

export const useProjectDragAndDropHandler = () => {
  const { projectsTrees: projectsTrees } = useProjectsTrees();

  const { mutateAsync: batchUpdateProject } = useBatchUpdateProject();

  const handleReorder = useCallback(
    async ({ location, source }) => {
      if (!projectsTrees || location.current?.dropTargets.length === 0) return;

      const sourceData = getTreeRootNodeSourceData(source);
      const targetData = getTreeRootNodeTargetData(location);

      if (targetData.data.projectId === sourceData.data.projectId) {
        return;
      }

      try {
        const sorted = [...projectsTrees].sort((a, b) => a.order! - b.order!);

        const sourceIndex = sorted.findIndex((p) => p.id === sourceData.data.projectId);
        const targetIndex = sorted.findIndex((p) => p.id === targetData.data.projectId);

        if (sourceIndex === -1 || targetIndex === -1) {
          console.error("Source or target project not found");
          return;
        }

        const insertAt = targetData.data.instruction.operation === "reorder-before" ? targetIndex : targetIndex + 1;

        const projectToMove = sorted[sourceIndex];

        const inserted = [
          ...sorted.slice(0, insertAt).filter((p) => p.id !== projectToMove.id),
          projectToMove,
          ...sorted.slice(insertAt).filter((p) => p.id !== projectToMove.id),
        ];

        const reordered = inserted.map((project, index) => ({
          ...project,
          order: index + 1,
        }));

        const projectsToUpdate = reordered.filter((reorderedProject) => {
          const projectUnderQuestion = sorted.find((sortedProject) => sortedProject.id === reorderedProject.id);
          return projectUnderQuestion!.order !== reorderedProject.order;
        });

        await batchUpdateProject({
          items: projectsToUpdate.map((project) => ({
            id: project.id,
            order: project.order,
          })),
        });
      } catch (error) {
        console.error("Error reordering projects:", error);
      }
    },
    [projectsTrees, batchUpdateProject]
  );

  useEffect(() => {
    return monitorForElements({
      canMonitor: ({ source }) => {
        return source.data.type === ProjectDragType.ROOT_NODE;
      },
      onDrop: handleReorder,
    });
  }, [handleReorder]);
};
