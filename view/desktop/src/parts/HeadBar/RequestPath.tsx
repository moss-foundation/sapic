import React, { useCallback, useEffect, useState } from "react";

import { ProjectTreeNode } from "@/components/ProjectTree/types";
import { useProjectsTrees } from "@/hooks/project/derivedHooks/useProjectsTrees";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamProjectsEvent } from "@repo/moss-workspace";

export interface RequestPathProps {
  className?: string;
}

export const RequestPath: React.FC<RequestPathProps> = ({ className = "" }) => {
  const { activePanelId, api } = useTabbedPaneStore();
  const { projectsTrees: projectsTrees } = useProjectsTrees();
  const [path, setPath] = useState<string[]>([]);
  const [activeTree, setActiveTree] = useState<ProjectTreeNode | null>(null);
  const [activeProject, setActiveProject] = useState<StreamProjectsEvent | null>(null);

  // Find path function (same logic as Breadcrumbs)
  const findPath = useCallback((node: ProjectTreeNode, target: string): string[] | null => {
    if (node.id === target) return [node.id];

    if (node.childNodes && node.childNodes.length > 0) {
      for (const child of node.childNodes) {
        const path = findPath(child, target);
        if (path) return [node.id.toString(), ...path];
      }
    }

    return null;
  }, []);

  useEffect(() => {
    if (!activePanelId) {
      setActiveTree(null);
      setPath([]);
      return;
    }

    const target = String(activePanelId);

    // Search through all projects to find the active panel
    for (const p of projectsTrees) {
      for (const category of p.childNodes) {
        const newPath = findPath(category, target);
        if (newPath) {
          setActiveTree(category);
          setActiveProject(p);
          setPath(newPath);
          return;
        }
      }
    }

    setActiveTree(null);
    setPath([]);
  }, [projectsTrees, activePanelId, findPath]);

  if (!activeTree || path.length === 0) {
    // Get current panel name when no project/endpoint is active
    const currentPanel = activePanelId ? api?.getPanel(activePanelId) : null;
    const panelTitle = currentPanel?.title || "";

    return (
      <span className={`truncate text-sm text-(--moss-headBar-icon-primary-text) ${className}`}>{panelTitle}</span>
    );
  }

  // Format the path for display - combine project name with path
  const formatPath = (pathSegments: string[]): string => {
    if (pathSegments.length === 0) return "";

    const projectName = activeProject?.name || "Project";

    const relevantSegments = pathSegments.slice(1);
    const pathString = relevantSegments.join("/");

    return `${projectName}/${pathString}`;
  };

  const displayPath = formatPath(path);

  return <span className={`truncate text-sm text-(--moss-headBar-icon-primary-text) ${className}`}>{displayPath}</span>;
};

export default RequestPath;
