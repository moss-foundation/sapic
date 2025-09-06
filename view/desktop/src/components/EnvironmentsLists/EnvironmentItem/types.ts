import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/closest-edge";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

export type EnvironmentListType = "GlobalEnvironmentItem" | "GroupedEnvironmentItem";

export interface DragEnvironmentItem {
  type: EnvironmentListType;
  data: {
    environment: StreamEnvironmentsEvent;
  };
  [key: string | symbol]: unknown;
}

export interface DropEnvironmentItem {
  type: EnvironmentListType;
  data: {
    environment: StreamEnvironmentsEvent;
  };
  edge?: Edge;
  [key: string | symbol]: unknown;
}
