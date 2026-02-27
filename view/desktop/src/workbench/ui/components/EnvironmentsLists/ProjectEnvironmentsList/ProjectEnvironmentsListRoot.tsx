import { useContext, useRef } from "react";

import { useListProjects } from "@/adapters/tanstackQuery/project/useListProjects";
import { useGetProjectEnvironments } from "@/db/environmentsSummaries/hooks/useGetProjectEnvironments";
import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useGetEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/useGetEnvironmentListItemState";

import { ProjectTreeContext } from "../../ProjectTree/ProjectTreeContext";
import { useDropTargetProjectEnvironmentList } from "../dnd/hooks/useDropTargetProjectEnvironmentList";
import { EnvironmentItem } from "../EnvironmentItem/EnvironmentItem";
import { ProjectEnvironmentsListRootHeaderDetails } from "./ProjectEnvironmentsListRootHeaderDetails";

interface ProjectEnvironmentsListRootProps {
  projectId: string;
}

export const ProjectEnvironmentsListRoot = ({ projectId }: ProjectEnvironmentsListRootProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { treePaddingLeft } = useContext(ProjectTreeContext);

  const projectEnvironmentsListRef = useRef<HTMLDivElement>(null);

  const { data: projects } = useListProjects();
  const { projectEnvironments } = useGetProjectEnvironments(projectId);

  const project = projects?.items.find((project) => project.id === projectId);
  const { data: expanded = false } = useGetEnvironmentListItemState(projectId, currentWorkspaceId);

  const listHeaderOffset = treePaddingLeft * 2;

  const { instruction } = useDropTargetProjectEnvironmentList({
    refList: projectEnvironmentsListRef,
    projectId,
    projectEnvironments: projectEnvironments ?? [],
  });

  if (!project) {
    console.error(`Project ${projectId} not found`);
    return null;
  }

  return (
    <Tree.List ref={projectEnvironmentsListRef} instruction={instruction}>
      <Tree.ListHeader offsetLeft={listHeaderOffset}>
        <ProjectEnvironmentsListRootHeaderDetails
          project={project}
          expanded={expanded}
          count={projectEnvironments?.length ?? 0}
        />
      </Tree.ListHeader>

      {expanded && (
        <Tree.RootNodeChildren>
          {projectEnvironments?.map((environment) => (
            <EnvironmentItem key={environment.id} environment={environment} />
          ))}
        </Tree.RootNodeChildren>
      )}
    </Tree.List>
  );
};
