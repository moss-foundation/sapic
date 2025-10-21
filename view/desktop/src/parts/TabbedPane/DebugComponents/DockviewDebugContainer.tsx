import { DockviewApi } from "moss-tabs";
import { useEffect } from "react";

import { GridActions } from "./gridActions";
import { GroupActions } from "./groupActions";
import { PanelActions } from "./panelActions";

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

const DockviewDebugContainer = ({
  api,
  panels,
  activePanel,
  groups,
  activeGroup,
  toggleCustomWatermark,
  hasCustomWatermark,
  toggleDebug,
  toggleLogs,
  showLogs,
}: {
  api: DockviewApi | undefined;
  panels: string[];
  activePanel: string | undefined;
  groups: string[];
  activeGroup: string | undefined;
  toggleCustomWatermark: () => void;
  hasCustomWatermark: boolean;
  toggleDebug: () => void;
  toggleLogs: () => void;
  showLogs: boolean;
}) => {
  useEffect(() => {
    loadMaterialSymbols();
  }, []);

  return (
    <div className="border-b border-(--moss-border) bg-[#0f162d] text-white">
      <div>
        <GridActions api={api} toggleCustomWatermark={toggleCustomWatermark} hasCustomWatermark={hasCustomWatermark} />
        {api && <PanelActions api={api} panels={panels} activePanel={activePanel} />}
        {api && <GroupActions api={api} groups={groups} activeGroup={activeGroup} />}
      </div>
      <div className="action-container flex items-center justify-end p-1 select-none">
        <button className="mr-2 rounded" onClick={toggleDebug}>
          <span className="material-symbols-outlined">engineering</span>
        </button>
        <button className="rounded p-1" onClick={toggleLogs}>
          <span className="pr-1">{`${showLogs ? "Hide" : "Show"} Events Log`}</span>
          <span className="material-symbols-outlined">terminal</span>
        </button>
      </div>
    </div>
  );
};

export default DockviewDebugContainer;
