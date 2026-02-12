import { useContext, useRef } from "react";

import { useStreamProjects } from "@/adapters";
import { useGetProjectEnvironments } from "@/db/environmentsSummaries/hooks/useGetProjectEnvironments";
import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useGetEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/useGetEnvironmentListItemState";

import { ProjectTreeContext } from "../../ProjectTree/ProjectTreeContext";
import { useDropTargetProjectEnvironmentList } from "../dnd/hooks/useDropTargetProjectEnvironmentList";
import { EnvironmentItem } from "../EnvironmentItem/EnvironmentItem";
import { ProjectEnvironmentsListRootControls } from "./ProjectEnvironmentsListRootControls";

interface ProjectEnvironmentsListRootProps {
  projectId: string;
}

export const ProjectEnvironmentsListRoot = ({ projectId }: ProjectEnvironmentsListRootProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const projectEnvironmentsListRef = useRef<HTMLUListElement>(null);

  const { data: projects } = useStreamProjects();
  const { projectEnvironments } = useGetProjectEnvironments(projectId);
  const { data: projectEnvironmentListItemState } = useGetEnvironmentListItemState(projectId, currentWorkspaceId);

  const project = projects?.find((project) => project.id === projectId);
  const expanded = projectEnvironmentListItemState?.expanded ?? false;

  const { instruction } = useDropTargetProjectEnvironmentList({
    refList: projectEnvironmentsListRef,
    projectId,
    projectEnvironments: projectEnvironments ?? [],
  });

  if (!project) return null;

  return (
    <Tree.RootNode ref={projectEnvironmentsListRef} combineInstruction={instruction} className={cn("cursor-pointer")}>
      <Tree.RootNodeHeader className="cursor-pointer">
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
