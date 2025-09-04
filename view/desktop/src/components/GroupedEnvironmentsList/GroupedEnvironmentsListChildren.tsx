import { Tree } from "@/lib/ui/Tree";

import { GroupedEnvironmentsListItem } from "./GroupedEnvironmentsListItem/GroupedEnvironmentsListItem";
import { GroupedWithEnvironment } from "./types";

interface GroupedEnvironmentsListChildrenProps {
  groupedWithEnvironments: GroupedWithEnvironment;
}

export const GroupedEnvironmentsListChildren = ({ groupedWithEnvironments }: GroupedEnvironmentsListChildrenProps) => {
  return (
    <Tree.RootNodeChildren hideDirDepthIndicator>
      {groupedWithEnvironments.environments.map((environment) => (
        <GroupedEnvironmentsListItem key={environment.id} environment={environment} />
      ))}
    </Tree.RootNodeChildren>
  );
};
