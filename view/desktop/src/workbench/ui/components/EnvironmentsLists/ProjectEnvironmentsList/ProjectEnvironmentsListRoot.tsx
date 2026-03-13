import { useRef } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useGetEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/useGetEnvironmentListItemState";

import { NODE_OFFSET, TREE_HEADER_PADDING_RIGHT } from "../../ProjectTree/constants";
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

const listHeaderOffset = NODE_OFFSET * 3;
const listItemOffset = NODE_OFFSET * 4;
const listItemOffsetForAddForm = NODE_OFFSET * 7;

export const ProjectEnvironmentsListRoot = ({ tree }: ProjectEnvironmentsListRootProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const projectEnvironmentsListRef = useRef<HTMLDivElement>(null);

  const { data: expanded = false } = useGetEnvironmentListItemState(tree.id, currentWorkspaceId);

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
      <Tree.ListHeader paddingLeft={listHeaderOffset} paddingRight={TREE_HEADER_PADDING_RIGHT}>
        <ProjectEnvironmentsListRootHeaderDetails
          project={tree}
          expanded={expanded}
          count={tree.environmentsList?.length ?? 0}
        />

        <ProjectEnvironmentsListRootHeaderActions setIsAddingProjectEnvironment={setIsAddingProjectEnvironment} />
      </Tree.ListHeader>

      {showChildren && (
        <Tree.RootChildren hideDirDepthIndicator>
          {tree.environmentsList?.map((environment) => (
            <EnvironmentItem key={environment.id} environment={environment} offsetLeft={listItemOffset} />
          ))}
        </Tree.RootChildren>
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
