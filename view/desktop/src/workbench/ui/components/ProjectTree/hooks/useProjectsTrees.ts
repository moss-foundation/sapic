import { useMemo } from "react";

import { useGetAllProjectEnvironments } from "@/db/environmentsSummaries/hooks/useGetAllProjectEnvironments";
import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { useGetAllLocalResourceSummaries } from "@/db/resourceSummaries/hooks/useGetAllLocalResourceSummaries";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { ProjectTreeRoot } from "@/workbench/ui/components/ProjectTree/types";

import { buildResourcesTree } from "../utils/buildResourcesTree";

export interface UseProjectsTreesDataResult {
  projectsTrees: ProjectTreeRoot[];
  projectsTreesSortedByOrder: ProjectTreeRoot[];
}

export const useProjectsTrees = (): UseProjectsTreesDataResult => {
  const { data: localProjectSummaries = [] } = useGetAllLocalProjectSummaries();
  const { data: localResourceSummaries = [] } = useGetAllLocalResourceSummaries();
  const { projectEnvironments = [] } = useGetAllProjectEnvironments();

  const projectsTrees = useMemo(() => {
    if (localProjectSummaries.length === 0) return [];

    return localProjectSummaries.map(
      (projectSummary): ProjectTreeRoot => ({
        ...projectSummary,
        id: projectSummary.id,
        name: projectSummary.name,
        expanded: projectSummary.expanded,
        archived: projectSummary.archived,
        branch: projectSummary.branch ?? undefined,
        iconPath: projectSummary.iconPath ?? undefined,
        resourcesTree: buildResourcesTree({ projectId: projectSummary.id, localResourceSummaries }),
        environmentsList: projectEnvironments.filter((env) => env.projectId === projectSummary.id),
      })
    );
  }, [localProjectSummaries, localResourceSummaries, projectEnvironments]);

  const projectsTreesSortedByOrder = useMemo(() => {
    return sortObjectsByOrder(projectsTrees);
  }, [projectsTrees]);

  return {
    projectsTrees,
    projectsTreesSortedByOrder,
  };
};
