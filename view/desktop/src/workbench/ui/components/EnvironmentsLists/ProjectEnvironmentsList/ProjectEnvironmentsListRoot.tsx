import { useRef } from "react";

import { useStreamProjects } from "@/adapters";
import { useGetProjectEnvironments } from "@/db/environmentsSummaries/hooks/useGetProjectEnvironments";
import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useGetEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/useGetEnvironmentListItemState";

import { ProjectEnvironmentsListChildren } from "./ProjectEnvironmentsListChildren";
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

  if (!project || projectEnvironments?.length === 0) return null;

  return (
    <Tree.RootNode
      ref={projectEnvironmentsListRef}
      //   instruction={instruction}
      //   combineInstruction={instruction}
      className={cn("cursor-pointer")}
      //   isDragging={isDragging}
    >
      <Tree.RootNodeHeader isActive={false} className="cursor-pointer">
        <ProjectEnvironmentsListRootControls project={project} expanded={expanded} />
      </Tree.RootNodeHeader>

      {expanded && <ProjectEnvironmentsListChildren environments={projectEnvironments ?? []} />}
    </Tree.RootNode>
  );
};
