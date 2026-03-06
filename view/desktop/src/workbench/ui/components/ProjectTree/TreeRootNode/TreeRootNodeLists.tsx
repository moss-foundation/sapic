import { useContext } from "react";

import { ProjectEnvironmentsListRoot } from "../../EnvironmentsLists/ProjectEnvironmentsList/ProjectEnvironmentsListRoot";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ResourcesTree } from "../ResourcesTree/ResourcesTree";

interface TreeRootNodeListsProps {
  isAddingRootFileNode: boolean;
  isAddingRootFolderNode: boolean;
  handleRootAddFormSubmit: (name: string) => void;
  handleRootAddFormCancel: () => void;
}

export const TreeRootNodeLists = ({
  isAddingRootFileNode,
  isAddingRootFolderNode,
  handleRootAddFormSubmit,
  handleRootAddFormCancel,
}: TreeRootNodeListsProps) => {
  const { id } = useContext(ProjectTreeContext);

  return (
    <div className="flex flex-col gap-1">
      <ProjectEnvironmentsListRoot projectId={id} />

      <ResourcesTree
        isAddingRootFileNode={isAddingRootFileNode}
        isAddingRootFolderNode={isAddingRootFolderNode}
        handleRootAddFormSubmit={handleRootAddFormSubmit}
        handleRootAddFormCancel={handleRootAddFormCancel}
      />
    </div>
  );
};
