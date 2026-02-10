import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Availability } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DragEnvironmentItem } from "../types.dnd";

interface CanCombineToEnvironmentListProps {
  environments: EnvironmentSummary[];
  sourceData: DragEnvironmentItem;
}

export const canCombineToEnvironmentList = ({
  environments,
  sourceData,
}: CanCombineToEnvironmentListProps): Availability => {
  const hasSameId = environments.some((env) => env.id === sourceData.data.id);
  if (hasSameId) {
    return "not-available";
  }

  const hasSameName = environments.some((env) => env.name === sourceData.data.name);
  if (hasSameName) {
    return "blocked";
  }

  return "available";
};
