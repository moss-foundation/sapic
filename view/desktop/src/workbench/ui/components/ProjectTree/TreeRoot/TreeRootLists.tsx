import { ProjectEnvironmentsListRoot } from "../../EnvironmentsLists/ProjectEnvironmentsList/ProjectEnvironmentsListRoot";
import { ResourcesTree } from "../ResourcesTree/ResourcesTree";
import { ProjectTree } from "../types";

interface TreeRootListsProps {
  tree: ProjectTree;
}

export const TreeRootLists = ({ tree }: TreeRootListsProps) => {
  return (
    <div className="flex flex-col">
      <ProjectEnvironmentsListRoot tree={tree} />

      <ResourcesTree tree={tree.resourcesTree} />
    </div>
  );
};
