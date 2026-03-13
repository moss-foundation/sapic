import { ResourceNode } from "../ResourcesTree/types";

export interface ResourcesTreeRoot {
  id: string;
  projectId: string;
  childNodes: ResourceNode[];
}
