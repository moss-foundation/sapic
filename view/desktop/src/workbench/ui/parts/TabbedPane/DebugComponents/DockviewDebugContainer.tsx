import { useEffect } from "react";

import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { GridActions } from "./gridActions";
import { GroupActions } from "./groupActions";
import { PanelActions } from "./panelActions";

const DockviewDebugContainer = () => {
  const { setShowDebugPanels, showDebugPanels } = useTabbedPaneStore();

  useEffect(() => {
    loadMaterialSymbols();
  }, []);

  const toggleDebugPanels = () => {
    setShowDebugPanels(!showDebugPanels);
  };

  return (
    <div className="border-(--moss-border) border-b bg-[#0f162d] text-white">
      <div>
        <GridActions />
        <PanelActions />
        <GroupActions />
      </div>
      <div className="action-container flex select-none items-center justify-end p-1">
        <button className="mr-2 cursor-pointer rounded" onClick={toggleDebugPanels} title="Toggle Debug Panels">
          <span className="material-symbols-outlined">engineering</span>
        </button>
      </div>
    </div>
  );
};

// Load Material Symbols font for debug components
const loadMaterialSymbols = () => {
  // Check if font is already loaded to avoid duplicates
  const existingLink = document.querySelector('link[href*="Material+Symbols+Outlined"]');
  if (existingLink) {
    return;
  }

  const link = document.createElement("link");
  link.rel = "stylesheet";
  link.href =
    "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200";
  link.onerror = () => console.warn("Failed to load Material Symbols font");
  document.head.appendChild(link);
};

export default DockviewDebugContainer;
