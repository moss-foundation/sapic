import { useEffect } from "react";

import { resourceService } from "@/domains/resource/resourceService";

interface useSyncResourceDetailsProps {
  resourceId: string;
  projectId: string;
}

export const useSyncResourceDetails = ({ resourceId, projectId }: useSyncResourceDetailsProps) => {
  useEffect(() => {
    resourceService.describe(projectId, resourceId);
  }, [projectId, resourceId]);
};
