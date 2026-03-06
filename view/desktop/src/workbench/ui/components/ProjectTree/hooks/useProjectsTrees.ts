import { useMemo } from "react";

import { useGetAllProjectEnvironments } from "@/db/environmentsSummaries/hooks/useGetAllProjectEnvironments";
import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { useSyncProjectSummaries } from "@/db/projectSummaries/hooks/useSyncProjectSummaries";
import { useGetAllLocalResourceSummaries } from "@/db/resourceSummaries/hooks/useGetAllLocalResourceSummaries";
import { useSyncResourceSummaries } from "@/db/resourceSummaries/hooks/useSyncResourceSummaries";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { ProjectTree } from "@/workbench/ui/components/ProjectTree/types";

import { buildResourcesTree } from "../utils/buildResourcesTree";

export interface UseProjectsTreesProps {
  projectsTrees: ProjectTree[];
  projectsTreesSortedByOrder: ProjectTree[];
  isLoading: boolean;
}

export const useProjectsTrees = (): UseProjectsTreesProps => {
  const { isPending: areProjectsPending } = useSyncProjectSummaries();
  const { isPending: areResourcesPending } = useSyncResourceSummaries();

  const { data: localProjectSummaries = [] } = useGetAllLocalProjectSummaries();
  const { data: localResourceSummaries = [] } = useGetAllLocalResourceSummaries();
  const { projectEnvironments = [] } = useGetAllProjectEnvironments();

  const isLoading = areResourcesPending || areProjectsPending;

  const projectsTrees = useMemo(() => {
    if (isLoading || localProjectSummaries.length === 0) return [];

    return localProjectSummaries.map(
      (projectSummary): ProjectTree => ({
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
  }, [isLoading, localProjectSummaries, localResourceSummaries, projectEnvironments]);

  const projectsTreesSortedByOrder = useMemo(() => {
    return sortObjectsByOrder(projectsTrees);
  }, [projectsTrees]);

  return {
    projectsTrees,
    projectsTreesSortedByOrder,
    isLoading,
  };
};
