import { useSyncProjectSummaries } from "@/db/projectSummaries/hooks/useSyncProjectSummaries";
import { useSyncResourceSummaries } from "@/db/resourceSummaries/hooks/useSyncResourceSummaries";
import { ProjectTree } from "@/workbench/ui/components/ProjectTree/types";

import { useProjectsTreesData } from "./useProjectsTreesData";

export interface UseProjectsTreesProps {
  projectsTrees: ProjectTree[];
  projectsTreesSortedByOrder: ProjectTree[];
  isLoading: boolean;
}

export const useProjectsTrees = (): UseProjectsTreesProps => {
  const { isPending: areProjectsPending } = useSyncProjectSummaries();
  const { isLoading: areResourcesLoading } = useSyncResourceSummaries();

  const { projectsTrees, projectsTreesSortedByOrder } = useProjectsTreesData();

  const isLoading = areResourcesLoading || areProjectsPending;

  return {
    projectsTrees: isLoading ? [] : projectsTrees,
    projectsTreesSortedByOrder: isLoading ? [] : projectsTreesSortedByOrder,
    isLoading,
  };
};
