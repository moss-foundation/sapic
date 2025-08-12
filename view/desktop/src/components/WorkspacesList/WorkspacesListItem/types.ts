import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/closest-edge";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

export interface DragWorkspacesListItem {
  type: "WorkspacesListItem";
  data: {
    environment: StreamEnvironmentsEvent;
  };
  [key: string | symbol]: unknown;
}

export interface DropWorkspacesListItem {
  type: "WorkspacesListItem";
  data: {
    environment: StreamEnvironmentsEvent;
  };
  edge: Edge;
  [key: string | symbol]: unknown;
}
