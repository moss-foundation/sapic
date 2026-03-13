import { ProjectEnvironmentsListRoot } from "../../EnvironmentsLists/ProjectEnvironmentsList/ProjectEnvironmentsListRoot";
import { ResourcesTree } from "../ResourcesTree/ResourcesTree";
import { ProjectTree } from "../types";

interface TreeRootNodeListsProps {
  tree: ProjectTree;
}

export const TreeRootNodeLists = ({ tree }: TreeRootNodeListsProps) => {
  return (
    <div className="flex flex-col">
      <ProjectEnvironmentsListRoot tree={tree} />

      <ResourcesTree tree={tree.resourcesTree} />
    </div>
  );
};
