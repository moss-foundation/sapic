import { ProjectEnvironmentsListRoot } from "../../EnvironmentsLists/ProjectEnvironmentsList/ProjectEnvironmentsListRoot";
import { ResourcesTree } from "../ResourcesTree/ResourcesTree";
import { ProjectTree } from "../types";

interface TreeRootNodeListsProps {
  tree: ProjectTree;
  isAddingRootFileNode: boolean;
  isAddingRootFolderNode: boolean;
  handleRootAddFormSubmit: (name: string) => void;
  handleRootAddFormCancel: () => void;
}

export const TreeRootNodeLists = ({
  tree,
  isAddingRootFileNode,
  isAddingRootFolderNode,
  handleRootAddFormSubmit,
  handleRootAddFormCancel,
}: TreeRootNodeListsProps) => {
  return (
    <div className="flex flex-col gap-1">
      <ProjectEnvironmentsListRoot tree={tree} />

      <ResourcesTree
        tree={tree.resourcesTree}
        isAddingRootFileNode={isAddingRootFileNode}
        isAddingRootFolderNode={isAddingRootFolderNode}
        handleRootAddFormSubmit={handleRootAddFormSubmit}
        handleRootAddFormCancel={handleRootAddFormCancel}
      />
    </div>
  );
};
