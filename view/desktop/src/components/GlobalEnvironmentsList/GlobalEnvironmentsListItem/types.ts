import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/closest-edge";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

export interface DragGlobalEnvironmentsListItem {
  type: "GlobalEnvironmentsListItem";
  data: {
    environment: StreamEnvironmentsEvent;
  };
  [key: string | symbol]: unknown;
}

export interface DropGlobalEnvironmentsListItem {
  type: "GlobalEnvironmentsListItem";
  data: {
    environment: StreamEnvironmentsEvent;
  };
  edge?: Edge;
  [key: string | symbol]: unknown;
}
