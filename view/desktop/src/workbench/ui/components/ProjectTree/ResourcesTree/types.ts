import { ListProjectResourceItem } from "@repo/ipc";

export interface ResourceNode extends ListProjectResourceItem {
  order?: number | undefined;
  expanded: boolean;
  childNodes: ResourceNode[];
}
