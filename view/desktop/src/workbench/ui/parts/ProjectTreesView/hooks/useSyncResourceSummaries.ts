import { useEffect } from "react";

import { useStreamedProjectsWithResources } from "@/adapters";
import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";

export const useSyncResourceSummaries = () => {
  const { data: projectsWithResources } = useStreamedProjectsWithResources();

  useEffect(() => {
    projectsWithResources.forEach((project) => {
      project.resources.forEach((resource) => {
        const hasResourceSummary = resourceSummariesCollection.has(resource.id);

        if (hasResourceSummary) {
          resourceSummariesCollection.update(resource.id, (draft) => {
            Object.assign(draft, {
              ...resource,
              protocol: resource.protocol ?? undefined,
              metadata: {
                isDirty: false,
              },
            });
          });
        } else {
          resourceSummariesCollection.insert({
            id: resource.id,
            name: resource.name,
            path: resource.path,
            class: resource.class,
            kind: resource.kind,
            protocol: resource.protocol ?? undefined,
            order: resource.order,
            expanded: resource.expanded,
            metadata: {
              isDirty: false,
            },
          });
        }
      });
    });
  }, [projectsWithResources]);
};
