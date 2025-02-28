import { NodeProps } from "./types";

type SortTypes = "none" | "order" | "alphabetically";

export const sortNodes = (nodes: NodeProps[], sortBy: SortTypes = "alphabetically"): NodeProps[] => {

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