import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { DropNodeElement, NodeProps, SortTypes, TreeNodeProps } from "./types";

export const updateTreeNode = (node: TreeNodeProps, updatedNode: TreeNodeProps): TreeNodeProps => {
    if (node.uniqueId === updatedNode.uniqueId) {
        return updatedNode;
    }
    return {
        ...node,
        childNodes: node.childNodes.map((child) => updateTreeNode(child, updatedNode)),
    };
};

export const sortNode = (node: TreeNodeProps, sortBy: SortTypes = "alphabetically"): TreeNodeProps => {
    return {
        ...node,
        childNodes: sortNodes(node.childNodes.map(child => sortNode(child, sortBy)), sortBy)
    }
}

export const sortNodes = (nodes: TreeNodeProps[], sortBy: SortTypes = "alphabetically"): TreeNodeProps[] => {
    if (sortBy === "alphabetically") {
        nodes.sort((a, b) => {
            if (a.isFolder && !b.isFolder) return -1;
            if (!a.isFolder && b.isFolder) return 1;
            if (a.id < b.id) return -1;
            if (a.id > b.id) return 1;
            return 0;
        });

        return nodes.map((node, index) => {
            return {
                ...node,
                order: index + 1
            }
        });
    }

    if (sortBy === "order") {
        nodes.sort((a, b) => a.order - b.order);
        return nodes;
    }

    return nodes;
};


export const addUniqueIdToTree = (tree: NodeProps): TreeNodeProps => {
    const id = "TreeNodeUniqueId-" + Math.random().toString(36).substring(2, 15);
    return {
        uniqueId: id,
        ...tree,
        childNodes: tree.childNodes.map(child => addUniqueIdToTree(child))
    }
}

export const removeUniqueIdFromTree = (tree: TreeNodeProps): NodeProps => {
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const { uniqueId, ...treeWithoutUniqueId } = tree

    return {
        ...treeWithoutUniqueId,
        childNodes: tree.childNodes.map(child => removeUniqueIdFromTree(child))
    }
}

export const findNodeByUniqueId = (tree: TreeNodeProps, uniqueId: string): TreeNodeProps | undefined => {
    if (tree.uniqueId === uniqueId) {
        return tree;
    }

    return tree.childNodes.find(child => findNodeByUniqueId(child, uniqueId));
}

export const findParentNodeByChildUniqueId = (
    tree: TreeNodeProps,
    uniqueId: string
): TreeNodeProps | undefined => {
    if (tree.childNodes.some(child => child.uniqueId === uniqueId)) {
        return tree;
    }

    for (const child of tree.childNodes) {
        const parent = findParentNodeByChildUniqueId(child, uniqueId);

        if (parent !== undefined) {
            return parent;
        }
    }

    return undefined;
};

export const hasDescendant = (tree: TreeNodeProps, node: TreeNodeProps): boolean => {
    if (!tree.childNodes) return false;
    return tree.childNodes.some((child) => child.uniqueId === node.uniqueId || hasDescendant(child, node));
};

export const hasDirectDescendant = (tree: TreeNodeProps, node: TreeNodeProps): boolean => {
    if (!tree.childNodes) return false;
    return tree.childNodes.some((child) => child.uniqueId === node.uniqueId && child.id === node.id);
};

export const hasDirectSimilarDescendant = (tree: TreeNodeProps, node: TreeNodeProps): boolean => {
    if (!tree.childNodes) return false;
    return tree.childNodes.some((child) => child.uniqueId === node.uniqueId || child.id === node.id);
};


export const removeNodeFromTree = (tree: TreeNodeProps, uniqueId: string): TreeNodeProps => {
    if (tree.childNodes.some(child => child.uniqueId === uniqueId)) {
        return {
            ...tree,
            childNodes: tree.childNodes.filter(child => child.uniqueId !== uniqueId)
        };
    }

    return {
        ...tree,
        childNodes: tree.childNodes.map(child => removeNodeFromTree(child, uniqueId))
    };
};

export const addNodeToFolder = (tree: TreeNodeProps, targetUniqueId: string, nodeToAdd: TreeNodeProps): TreeNodeProps => {
    if (tree.uniqueId === targetUniqueId) {
        return {
            ...tree,
            childNodes: [...tree.childNodes, nodeToAdd]
        };
    }

    return {
        ...tree,
        childNodes: tree.childNodes.map(child => addNodeToFolder(child, targetUniqueId, nodeToAdd))
    };
};

export const getActualDropSourceTarget = (source: ElementDragPayload): DropNodeElement => {
    return source.data.data as DropNodeElement
}

export const getActualDropTarget = (location: DragLocationHistory): DropNodeElement => {
    return (location.current.dropTargets[0].data.data as DropNodeElement).node
        .isFolder
        ? (location.current.dropTargets[0].data.data as DropNodeElement)
        : (location.current.dropTargets[1].data.data as DropNodeElement);
}

export const canDrop = (sourceTarget: DropNodeElement, dropTarget: DropNodeElement, node: TreeNodeProps) => {
    if (sourceTarget.node.isFolder === false) {
        if (hasDirectSimilarDescendant(node, sourceTarget.node)) {
            return false;
        }
    }

    if (sourceTarget.node.isFolder) {
        if (hasDirectSimilarDescendant(node, sourceTarget.node)) {
            return false;
        }

        if (hasDirectDescendant(dropTarget.node, node)) {
            return false;
        }

        if (hasDescendant(sourceTarget.node, node)) {
            return false;
        }

        if (sourceTarget?.node.uniqueId === node.uniqueId) {
            return false;
        }
    }

    return true;
}

export const expandAllNodes = (node: TreeNodeProps): TreeNodeProps => {
    return {
        ...node,
        isExpanded: true,
        childNodes: node.childNodes.map(child => expandAllNodes(child))
    }
}

export const collapseAllNodes = (node: TreeNodeProps): TreeNodeProps => {
    return {
        ...node,
        isExpanded: false,
        childNodes: node.childNodes.map(child => collapseAllNodes(child))
    }
}

