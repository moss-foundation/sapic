import { ProjectEnvironmentsListRoot } from "../../EnvironmentsLists/ProjectEnvironmentsList/ProjectEnvironmentsListRoot";
import { ResourcesTree } from "../ResourcesTree/ResourcesTree";
import { ProjectTreeRoot } from "../types";

interface TreeRootListsProps {
  tree: ProjectTreeRoot;
}

export const TreeRootLists = ({ tree }: TreeRootListsProps) => {
  return (
    <div className="flex flex-col">
      <ProjectEnvironmentsListRoot tree={tree} />

      <ResourcesTree tree={tree.resourcesTree} />
    </div>
  );
};
