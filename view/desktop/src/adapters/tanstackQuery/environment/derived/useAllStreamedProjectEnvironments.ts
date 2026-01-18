import { useCallback, useEffect, useState } from "react";

import { environmentService } from "@/domains/environment/environmentService";
import { StreamEnvironmentsEvent } from "@repo/ipc";

import { useStreamProjects } from "../../project";

export const useAllStreamedProjectEnvironments = () => {
  const { data: projects } = useStreamProjects();

  const [allProjectEnvironments, setAllProjectEnvironments] = useState<StreamEnvironmentsEvent[]>([]);

  useEffect(() => {
    if (!projects) return;

    const fetchAllProjectEnvironments = async () => {
      const promises = projects.map(async (project) => {
        return await environmentService.streamProjectEnvironments({
          projectId: project.id,
        });
      });

      const results = await Promise.all(promises);
      setAllProjectEnvironments(results.flat());
    };

    fetchAllProjectEnvironments();
  }, [projects]);

  const refetch = useCallback(() => {
    if (!projects) return;

    setAllProjectEnvironments([]);

    const fetchAllProjectEnvironments = async () => {
      const promises = projects.map(async (project) => {
        return await environmentService.streamProjectEnvironments({
          projectId: project.id,
        });
      });

      const results = await Promise.all(promises);
      setAllProjectEnvironments(results.flat());
    };

    fetchAllProjectEnvironments();
  }, [projects]);

  return { allProjectEnvironments, refetch };
};
