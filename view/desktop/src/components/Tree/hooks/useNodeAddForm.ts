import { useState } from "react";
import { NodeProps, TreeNodeProps } from "../types";
import { sortNodes, addUniqueIdToTree } from "../utils";

export const useNodeAddForm = (node: TreeNodeProps, onNodeUpdate: (node: TreeNodeProps) => void) => {
    const [isAddingFileNode, setIsAddingFileNode] = useState(false);
    const [isAddingFolderNode, setIsAddingFolderNode] = useState(false);

    const handleAddFormSubmit = (newNode: NodeProps) => {
        onNodeUpdate({
            ...node,
            isExpanded: true,
            childNodes: sortNodes([...node.childNodes, addUniqueIdToTree(newNode)])
        });

        setIsAddingFileNode(false);
        setIsAddingFolderNode(false);
    };

    const handleAddFormCancel = () => {
        setIsAddingFileNode(false);
        setIsAddingFolderNode(false);
    };

    return {
        isAddingFileNode,
        isAddingFolderNode,
        setIsAddingFileNode,
        setIsAddingFolderNode,
        handleAddFormSubmit,
        handleAddFormCancel
    }
}