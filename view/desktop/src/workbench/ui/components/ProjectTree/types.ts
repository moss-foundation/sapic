import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { ListProjectItem } from "@repo/ipc";

import { ResourcesTreeRoot } from "./TreeRoot/types";

export interface ProjectTreeRoot extends ListProjectItem {
  order?: number | undefined;
  expanded: boolean;
  resourcesTree: ResourcesTreeRoot;
  environmentsList: EnvironmentSummary[];
}
