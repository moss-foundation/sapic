import { useEffect } from "react";

import { useAllEnvironments } from "@/adapters/tanstackQuery/environment/derived/useAllEnvironments";
import { useCurrentWorkspace } from "@/hooks";
import { environmentItemStateService } from "@/workbench/services/environmentItemStateService";

import { flushEnvironmentSummaries } from "../actions/flushEnvironmentSummaries";
import { environmentSummariesCollection } from "../environmentSummaries";
import { EnvironmentSummary } from "../types";

export const useSyncEnvironments = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { data: allEnvironments, isPending: isAllEnvironmentsLoading } = useAllEnvironments();

  useEffect(flushEnvironmentSummaries, [currentWorkspaceId]);

  useEffect(() => {
    if (isAllEnvironmentsLoading || !allEnvironments) return;

    const syncEnvironments = async () => {
      const envIds = allEnvironments.map((env) => env.id);
      const [envOrders, envExpanded] = await Promise.all([
        environmentItemStateService.batchGetOrder(envIds, currentWorkspaceId),
        environmentItemStateService.batchGetExpanded(envIds, currentWorkspaceId),
      ]);

      const summaries: EnvironmentSummary[] = allEnvironments.map((env, index) => ({
        id: env.id,
        projectId: env.projectId,
        isActive: env.isActive,
        name: env.name,
        color: env.color,
        totalVariables: env.totalVariables,
        order: envOrders[index],
        expanded: envExpanded[index] ?? false,
      }));

      // Fill environment summaries
      summaries.forEach((summary) => {
        if (!environmentSummariesCollection.has(summary.id)) {
          environmentSummariesCollection.insert(summary);
        } else {
          environmentSummariesCollection.update(summary.id, (draft) => {
            Object.assign(draft, summary);
          });
        }
      });

      // Remove environment summaries that are not on the backend
      environmentSummariesCollection.forEach((summary) => {
        if (!summaries.some((s) => s.id === summary.id)) {
          environmentSummariesCollection.delete(summary.id);
        }
      });
    };

    syncEnvironments();
  }, [currentWorkspaceId, isAllEnvironmentsLoading, allEnvironments]);
};
