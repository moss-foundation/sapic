import { useMemo } from "react";

import { useStreamEnvironments } from "@/adapters/tanstackQuery/environment";
import { useStreamProjects } from "@/adapters/tanstackQuery/project";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";

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
