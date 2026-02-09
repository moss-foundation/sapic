import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Tree } from "@/lib/ui/Tree";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "../constants";
import { EnvironmentItem } from "../EnvironmentItem/EnvironmentItem";

interface ProjectEnvironmentsListChildrenProps {
  environments: EnvironmentSummary[];
}

export const ProjectEnvironmentsListChildren = ({ environments }: ProjectEnvironmentsListChildrenProps) => {
  return (
    <Tree.RootNodeChildren hideDirDepthIndicator>
      {environments.map((environment) => (
        <EnvironmentItem key={environment.id} environment={environment} type={ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT} />
      ))}
    </Tree.RootNodeChildren>
  );
};
