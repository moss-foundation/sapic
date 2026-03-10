import { useContext, useRef } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useGetEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/useGetEnvironmentListItemState";

import { ProjectTreeContext } from "../../ProjectTree/ProjectTreeContext";
import { ProjectTree } from "../../ProjectTree/types";
import { useDropTargetProjectEnvironmentList } from "../dnd/hooks/useDropTargetProjectEnvironmentList";
import { ProjectEnvironmentAddForm } from "../EnvironmentAddForm/ProjectEnvironmentAddForm";
import { EnvironmentItem } from "../EnvironmentItem/EnvironmentItem";
import { useAddProjectEnvironmentForm } from "./hooks/useAddProjectEnvironmentForm";
import { ProjectEnvironmentsListRootHeaderActions } from "./ProjectEnvironmentsListRootHeaderActions";
import { ProjectEnvironmentsListRootHeaderDetails } from "./ProjectEnvironmentsListRootHeaderDetails";

interface ProjectEnvironmentsListRootProps {
  tree: ProjectTree;
}

export const ProjectEnvironmentsListRoot = ({ tree }: ProjectEnvironmentsListRootProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { treePaddingLeft, treePaddingRight } = useContext(ProjectTreeContext);

  const projectEnvironmentsListRef = useRef<HTMLDivElement>(null);

  const { data: expanded = false } = useGetEnvironmentListItemState(tree.id, currentWorkspaceId);

  const listHeaderOffset = treePaddingLeft * 2;
  const listItemOffset = treePaddingLeft * 3;
  const listItemOffsetForAddForm = treePaddingLeft * 6;

  const { instruction } = useDropTargetProjectEnvironmentList({
    refList: projectEnvironmentsListRef,
    projectId: tree.id,
    projectEnvironments: tree.environmentsList ?? [],
  });

  const {
    isAddingProjectEnvironment,
    setIsAddingProjectEnvironment,
    handleAddProjectEnvironmentSubmit,
    handleAddProjectEnvironmentFormCancel,
  } = useAddProjectEnvironmentForm({
    environmentsList: tree.environmentsList ?? [],
  });

  const showChildren = expanded || isAddingProjectEnvironment;
  const restrictedNames = tree.environmentsList?.map((environment) => environment.name) ?? [];

  return (
    <Tree.List ref={projectEnvironmentsListRef} combineInstruction={instruction}>
      <Tree.ListHeader offsetLeft={listHeaderOffset} offsetRight={treePaddingRight}>
        <ProjectEnvironmentsListRootHeaderDetails
          project={tree}
          expanded={expanded}
          count={tree.environmentsList?.length ?? 0}
        />

        <ProjectEnvironmentsListRootHeaderActions setIsAddingProjectEnvironment={setIsAddingProjectEnvironment} />
      </Tree.ListHeader>

      {showChildren && (
        <Tree.RootNodeChildren hideDirDepthIndicator>
          {tree.environmentsList?.map((environment) => (
            <EnvironmentItem key={environment.id} environment={environment} offsetLeft={listItemOffset} />
          ))}
        </Tree.RootNodeChildren>
      )}

      {isAddingProjectEnvironment && (
        <ProjectEnvironmentAddForm
          offsetLeft={listItemOffsetForAddForm}
          onSubmit={handleAddProjectEnvironmentSubmit}
          restrictedNames={restrictedNames}
          onCancel={handleAddProjectEnvironmentFormCancel}
        />
      )}
    </Tree.List>
  );
};
