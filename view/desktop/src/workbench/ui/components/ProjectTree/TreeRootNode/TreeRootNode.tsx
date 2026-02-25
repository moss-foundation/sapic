import { useContext, useRef } from "react";

import { useListProjects } from "@/adapters/tanstackQuery/project/useListProjects";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { ProjectEnvironmentsListRoot } from "../../EnvironmentsLists/ProjectEnvironmentsList/ProjectEnvironmentsListRoot";
import { useDraggableRootNode } from "../dnd/hooks/useDraggableRootNode";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeRootNodeProps } from "../types";
import { calculateShouldRenderRootChildNodes } from "../utils";
import { useRootNodeAddForm } from "./hooks/useRootNodeAddForm";
import { useRootNodeRenamingForm } from "./hooks/useRootNodeRenamingForm";
import { TreeRootControls } from "./TreeRootControls";
import { TreeRootNodeRenamingForm } from "./TreeRootNodeRenamingForm";
import { TreeRootResourcesList } from "./TreeRootResourcesList";

export const TreeRootNode = ({ node }: ProjectTreeRootNodeProps) => {
  const { id, treePaddingLeft, treePaddingRight } = useContext(ProjectTreeContext);

  const draggableHeaderRef = useRef<HTMLLIElement>(null);
  const dropTargetRootRef = useRef<HTMLUListElement>(null);

  const { data: projects } = useListProjects();
  const { activePanelId } = useTabbedPaneStore();

  const {
    isAddingRootFileNode,
    isAddingRootFolderNode,
    setIsAddingRootFileNode,
    setIsAddingRootFolderNode,
    handleRootAddFormCancel,
    handleRootAddFormSubmit,
  } = useRootNodeAddForm(node);

  const {
    isRenamingRootNode,
    setIsRenamingRootNode,
    handleRenamingRootNodeFormSubmit,
    handleRenamingRootNodeFormCancel,
  } = useRootNodeRenamingForm(node);

  const { isDragging, instruction } = useDraggableRootNode({
    nodeRef: dropTargetRootRef,
    triggerRef: draggableHeaderRef,
    node,
    isRenamingNode: isRenamingRootNode,
  });

  const shouldRenderLists = calculateShouldRenderRootChildNodes(node, isAddingRootFileNode, isRenamingRootNode);
  const restrictedNames = projects?.items.map((project) => project.name) ?? [];

  return (
    <Tree.RootNode ref={dropTargetRootRef} instruction={instruction} isDragging={isDragging}>
      <Tree.RootNodeHeader
        ref={draggableHeaderRef}
        isActive={activePanelId === node.id}
        treePaddingLeft={treePaddingLeft}
        treePaddingRight={treePaddingRight}
      >
        {isRenamingRootNode ? (
          <TreeRootNodeRenamingForm
            node={node}
            shouldRenderChildNodes={shouldRenderLists}
            restrictedNames={restrictedNames}
            handleRenamingFormSubmit={handleRenamingRootNodeFormSubmit}
            handleRenamingFormCancel={handleRenamingRootNodeFormCancel}
          />
        ) : (
          <TreeRootControls
            node={node}
            setIsAddingRootFileNode={setIsAddingRootFileNode}
            setIsAddingRootFolderNode={setIsAddingRootFolderNode}
            setIsRenamingRootNode={setIsRenamingRootNode}
          />
        )}
      </Tree.RootNodeHeader>

      {shouldRenderLists && (
        <div className="flex flex-col gap-1">
          <ProjectEnvironmentsListRoot projectId={id} />

          <TreeRootResourcesList
            tree={node}
            isAddingRootFileNode={isAddingRootFileNode}
            isAddingRootFolderNode={isAddingRootFolderNode}
            handleRootAddFormSubmit={handleRootAddFormSubmit}
            handleRootAddFormCancel={handleRootAddFormCancel}
          />
        </div>
      )}
    </Tree.RootNode>
  );
};
