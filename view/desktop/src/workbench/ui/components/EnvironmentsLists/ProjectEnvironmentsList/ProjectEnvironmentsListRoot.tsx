import { useRef } from "react";

import { useListProjects } from "@/adapters/tanstackQuery/project/useListProjects";
import { useGetProjectEnvironments } from "@/db/environmentsSummaries/hooks/useGetProjectEnvironments";
import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useGetEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/useGetEnvironmentListItemState";

import { useDropTargetProjectEnvironmentList } from "../dnd/hooks/useDropTargetProjectEnvironmentList";
import { EnvironmentItem } from "../EnvironmentItem/EnvironmentItem";
import { ProjectEnvironmentsListRootControls } from "./ProjectEnvironmentsListRootControls";

interface ProjectEnvironmentsListRootProps {
  projectId: string;
}

export const ProjectEnvironmentsListRoot = ({ projectId }: ProjectEnvironmentsListRootProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const projectEnvironmentsListRef = useRef<HTMLUListElement>(null);

  const { data: projects } = useListProjects();
  const { projectEnvironments } = useGetProjectEnvironments(projectId);

  const project = projects?.items.find((project) => project.id === projectId);
  const { data: projectEnvironmentListItemState } = useGetEnvironmentListItemState(projectId, currentWorkspaceId);
  const expanded = projectEnvironmentListItemState?.expanded ?? false;

  const { instruction } = useDropTargetProjectEnvironmentList({
    refList: projectEnvironmentsListRef,
    projectId,
    projectEnvironments: projectEnvironments ?? [],
  });

  if (!project) return null;

  return (
    <Tree.RootNode ref={projectEnvironmentsListRef} combineInstruction={instruction} className={cn("cursor-pointer")}>
      <Tree.RootNodeHeader disableIndicator={true}>
        <ProjectEnvironmentsListRootControls
          project={project}
          expanded={expanded}
          count={projectEnvironments?.length ?? 0}
        />
      </Tree.RootNodeHeader>

      {expanded && (
        <Tree.RootNodeChildren hideDirDepthIndicator>
          {projectEnvironments?.map((environment) => (
            <EnvironmentItem key={environment.id} environment={environment} />
          ))}
        </Tree.RootNodeChildren>
      )}
    </Tree.RootNode>
  );
};
