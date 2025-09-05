import { Tree } from "@/lib/ui/Tree";

import { EnvironmentListItem } from "../EnvironmentItem/EnvironmentListItem";
import { GroupedEnvironments } from "../types";

interface GroupedEnvironmentsListChildrenProps {
  groupedEnvironments: GroupedEnvironments;
}

export const GroupedEnvironmentsListChildren = ({ groupedEnvironments }: GroupedEnvironmentsListChildrenProps) => {
  return (
    <Tree.RootNodeChildren hideDirDepthIndicator>
      {groupedEnvironments.environments.map((environment) => (
        <EnvironmentListItem key={environment.id} environment={environment} type="grouped" />
      ))}
    </Tree.RootNodeChildren>
  );
};
