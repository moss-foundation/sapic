import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Tree } from "@/lib/ui/Tree";

import { EnvironmentItem } from "../EnvironmentItem/EnvironmentItem";
import { EnvironmentListType } from "../types";

interface ProjectEnvironmentsListChildrenProps {
  environments: EnvironmentSummary[];
}

export const ProjectEnvironmentsListChildren = ({ environments }: ProjectEnvironmentsListChildrenProps) => {
  return (
    <Tree.RootNodeChildren hideDirDepthIndicator>
      {environments.map((environment) => (
        <EnvironmentItem key={environment.id} environment={environment} type={EnvironmentListType.GROUPED} />
      ))}
    </Tree.RootNodeChildren>
  );
};
