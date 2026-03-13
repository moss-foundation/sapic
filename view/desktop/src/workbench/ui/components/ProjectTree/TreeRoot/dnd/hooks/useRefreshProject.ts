import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
import { resourceService } from "@/domains/resource/resourceService";

export const useRefreshProject = (projectId: string) => {
  const refreshProject = async () => {
    resourceSummariesCollection.forEach((resource) => {
      if (resource.projectId === projectId) {
        resourceSummariesCollection.delete(resource.id);
      }
    });

    await resourceService.list({ projectId, mode: { "RELOAD_PATH": "" } });
  };

  return { refreshProject };
};
