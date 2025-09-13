import React, { useEffect, useState } from "react";

import { ProjectTreeNode } from "@/components/ProjectTree/types";
import { useCollectionsTrees } from "@/hooks/collection/derivedHooks/useCollectionsTrees";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamCollectionsEvent } from "@repo/moss-workspace";

export interface RequestPathProps {
  className?: string;
}

export const RequestPath: React.FC<RequestPathProps> = ({ className = "" }) => {
  const { activePanelId, api } = useTabbedPaneStore();
  const { collectionsTrees } = useCollectionsTrees();
  const [path, setPath] = useState<string[]>([]);
  const [activeTree, setActiveTree] = useState<ProjectTreeNode | null>(null);
  const [activeCollection, setActiveCollection] = useState<StreamCollectionsEvent | null>(null);

  // Find path function (same logic as Breadcrumbs)
  const findPath = (node: ProjectTreeNode, target: string): string[] | null => {
    if (node.id === target) return [node.id];

    if (node.childNodes && node.childNodes.length > 0) {
      for (const child of node.childNodes) {
        const path = findPath(child, target);
        if (path) return [node.id.toString(), ...path];
      }
    }

    return null;
  };

  useEffect(() => {
    if (!activePanelId) {
      setActiveTree(null);
      setPath([]);
      return;
    }

    const target = String(activePanelId);

    // Search through all collections to find the active panel
    for (const collection of collectionsTrees) {
      const categories = [collection.requests, collection.endpoints, collection.components, collection.schemas];

      for (const category of categories) {
        const newPath = findPath(category, target);
        if (newPath) {
          setActiveTree(category);
          setActiveCollection(collection);
          setPath(newPath);
          return;
        }
      }
    }

    setActiveTree(null);
    setPath([]);
  }, [collectionsTrees, activePanelId]);

  if (!activeTree || path.length === 0) {
    // Get current panel name when no collection/request is active
    const currentPanel = activePanelId ? api?.getPanel(activePanelId) : null;
    const panelTitle = currentPanel?.title || "";

    return (
      <span className={`truncate text-sm text-(--moss-headBar-icon-primary-text) ${className}`}>{panelTitle}</span>
    );
  }

  // Format the path for display - combine collection name with path
  const formatPath = (pathSegments: string[]): string => {
    if (pathSegments.length === 0) return "";

    const collectionName = activeCollection?.name || "Collection";

    const relevantSegments = pathSegments.slice(1);
    const pathString = relevantSegments.join("/");

    return `${collectionName}/${pathString}`;
  };

  const displayPath = formatPath(path);

  return <span className={`truncate text-sm text-(--moss-headBar-icon-primary-text) ${className}`}>{displayPath}</span>;
};

export default RequestPath;
