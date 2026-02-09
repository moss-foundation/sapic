import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { EnvironmentItemState } from "@/workbench/domains/environmentItemState/types";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DragEnvironmentItem, DropEnvironmentItem } from "../types.dnd";

interface HandleReorderWorkspaceEnvsProps {
  sourceData: DragEnvironmentItem;
  locationData: DropEnvironmentItem;
  workspaceEnvironments: EnvironmentSummary[];
  batchPutEnvironmentItemState: (props: {
    environmentItemStates: EnvironmentItemState[];
    workspaceId: string;
  }) => Promise<void>;
  instruction: Instruction;
  currentWorkspaceId: string;
}

export const handleReorderWorkspaceEnvs = async ({
  sourceData,
  locationData,
  workspaceEnvironments,
  batchPutEnvironmentItemState,
  instruction,
  currentWorkspaceId,
}: HandleReorderWorkspaceEnvsProps) => {
  const targetIndex = workspaceEnvironments.findIndex((env) => env.id === locationData.data.id);
  const dropOrder = instruction.operation === "reorder-before" ? targetIndex : targetIndex + 1;

  const environmentsToUpdate = [
    ...workspaceEnvironments.slice(0, dropOrder).filter((env) => env.id !== sourceData.data.id),
    sourceData.data,
    ...workspaceEnvironments.slice(dropOrder).filter((env) => env.id !== sourceData.data.id),
  ].map((env, index) => ({
    id: env.id,
    order: index + 1,
  }));

  await batchPutEnvironmentItemState({
    workspaceId: currentWorkspaceId,
    environmentItemStates: environmentsToUpdate.map((env) => ({
      id: env.id,
      order: env.order,
    })),
  });
};
