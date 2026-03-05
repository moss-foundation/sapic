import { useContext } from "react";

import { ProjectEnvironmentsListRoot } from "../../EnvironmentsLists/ProjectEnvironmentsList/ProjectEnvironmentsListRoot";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ResourcesTree } from "../ResourcesTree/ResourcesTree";

export const TreeRootNodeLists = () => {
  const { id } = useContext(ProjectTreeContext);

  return (
    <div className="flex flex-col gap-1">
      <ProjectEnvironmentsListRoot projectId={id} />

      <ResourcesTree />
    </div>
  );
};
