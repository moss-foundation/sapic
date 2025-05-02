import { DockviewApi } from "@repo/moss-tabs";

import { GridActions } from "./gridActions";
import { GroupActions } from "./groupActions";
import { PanelActions } from "./panelActions";

const DockviewControls = ({
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
  return (
    <div className="border-b border-(--moss-border-color) bg-[#0f162d] text-white">
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

export default DockviewControls;
