import { useContext, useRef } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useGetEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/useGetEnvironmentListItemState";

import { ProjectTreeContext } from "../../ProjectTree/ProjectTreeContext";
import { ProjectTree } from "../../ProjectTree/types";
import { useDropTargetProjectEnvironmentList } from "../dnd/hooks/useDropTargetProjectEnvironmentList";
import { EnvironmentItem } from "../EnvironmentItem/EnvironmentItem";
import { ProjectEnvironmentsListRootHeaderDetails } from "./ProjectEnvironmentsListRootHeaderDetails";

interface ProjectEnvironmentsListRootProps {
  tree: ProjectTree;
}

export const ProjectEnvironmentsListRoot = ({ tree }: ProjectEnvironmentsListRootProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { treePaddingLeft } = useContext(ProjectTreeContext);

  const projectEnvironmentsListRef = useRef<HTMLDivElement>(null);

  const { data: expanded = false } = useGetEnvironmentListItemState(tree.id, currentWorkspaceId);

  const listHeaderOffset = treePaddingLeft * 2;
  const listItemOffset = treePaddingLeft * 3;

  const { instruction } = useDropTargetProjectEnvironmentList({
    refList: projectEnvironmentsListRef,
    projectId: tree.id,
    projectEnvironments: tree.environmentsList ?? [],
  });

  return (
    <Tree.List ref={projectEnvironmentsListRef} combineInstruction={instruction}>
      <Tree.ListHeader offsetLeft={listHeaderOffset}>
        <ProjectEnvironmentsListRootHeaderDetails
          project={tree}
          expanded={expanded}
          count={tree.environmentsList?.length ?? 0}
        />
      </Tree.ListHeader>

      {expanded && (
        <Tree.RootNodeChildren hideDirDepthIndicator>
          {tree.environmentsList?.map((environment) => (
            <EnvironmentItem key={environment.id} environment={environment} offsetLeft={listItemOffset} />
          ))}
        </Tree.RootNodeChildren>
      )}
    </Tree.List>
  );
};
