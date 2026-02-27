import { useContext } from "react";

import { ProjectEnvironmentsListRoot } from "../../EnvironmentsLists/ProjectEnvironmentsList/ProjectEnvironmentsListRoot";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeRootNode } from "../types";
import { TreeRootNodeResourcesList } from "./TreeRootNodeResourcesList";

interface TreeRootNodeListsProps {
  tree: ProjectTreeRootNode;
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
  const { id } = useContext(ProjectTreeContext);

  return (
    <div className="flex flex-col gap-1">
      <ProjectEnvironmentsListRoot projectId={id} />

      <TreeRootNodeResourcesList
        tree={tree}
        isAddingRootFileNode={isAddingRootFileNode}
        isAddingRootFolderNode={isAddingRootFolderNode}
        handleRootAddFormSubmit={handleRootAddFormSubmit}
        handleRootAddFormCancel={handleRootAddFormCancel}
      />
    </div>
  );
};
