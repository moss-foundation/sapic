import { useCallback, useEffect, useState } from "react";

import { useStreamProjects } from "@/adapters/tanstackQuery/project";
import { environmentService } from "@/domains/environment/environmentService";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";

import { GroupedEnvironments } from "../types";

//FIXME this whole hook is a mess and needs to be refactored
//It isnt shared between the components
export const useGroupedEnvironments = () => {
  const { data: projects } = useStreamProjects();

  const [groupedEnvironments, setGroupedEnvironments] = useState<GroupedEnvironments[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  const fetchAllProjectEnvironments = useCallback(async () => {
    if (!projects) return;

    setIsLoading(true);
    setGroupedEnvironments([]);

    const promises = Array.from(projects.entries()).map(async ([index, project]) => {
      const environments = await environmentService.streamProjectEnvironments({
        projectId: project.id,
      });

      return {
        projectId: project.id,
        expanded: true,
        order: index + 1,
        environments,
      };
    });

    const results = (await Promise.all(promises)).filter((result) => result.environments.length > 0);

    setGroupedEnvironments(sortObjectsByOrder(results));
    setIsLoading(false);
  }, [projects]);

  const refetchProjectEnvironments = useCallback(
    async (projectId: string) => {
      if (!projects) return;

      setIsLoading(true);

      const environments = await environmentService.streamProjectEnvironments({
        projectId,
      });

      setGroupedEnvironments((prev) =>
        prev.map((group) => (group.projectId === projectId ? { ...group, environments } : group))
      );
      setIsLoading(false);
    },
    [projects]
  );

  const refetchGroupedEnvironments = useCallback(async () => {
    await fetchAllProjectEnvironments();
  }, [fetchAllProjectEnvironments]);

  const clearGroupedEnvironments = () => {
    setGroupedEnvironments([]);
  };

  useEffect(() => {
    if (!projects) return;
    fetchAllProjectEnvironments();
  }, [projects, fetchAllProjectEnvironments]);

  return {
    groupedEnvironments,
    refetchGroupedEnvironments,
    isLoading,
    clearGroupedEnvironments,
    fetchAllProjectEnvironments,
    refetchProjectEnvironments,
  };
};
