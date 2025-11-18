import { useMemo } from "react";

import { useStreamProjects } from "@/hooks";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { useStreamEnvironments } from "@/workbench/adapters";

import { GroupedEnvironments } from "../types";

export const useGroupedEnvironments = () => {
  const { data: projects } = useStreamProjects();
  const { groups, projectEnvironments } = useStreamEnvironments();

  const groupedEnvironments: GroupedEnvironments[] = useMemo(() => {
    if (!projects || !groups || !projectEnvironments) return [];

    const groupedEnvironments = groups.map((group) => {
      return {
        ...group,
        environments: projectEnvironments.filter((environment) => environment.projectId === group.projectId),
      };
    });

    return sortObjectsByOrder(groupedEnvironments);
  }, [projectEnvironments, projects, groups]);

  return { groupedEnvironments };
};
