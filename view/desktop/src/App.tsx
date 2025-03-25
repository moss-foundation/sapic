import { useState } from "react";

import { Resizable, ResizablePanel } from "./components";
import CollectionTreeView from "./components/CollectionTreeView";
import { SideBar } from "./components/SideBar";
import { HeadBar } from "./parts/HeadBar/HeadBar";
import TabbedPane from "./parts/TabbedPane/TabbedPane";
import { useActivityBarStore } from "./store/activityBarStore";
import { usePositionStore } from "./store/positionStore";
import { useSideBarStore } from "./store/sideBarStore";

import "@repo/moss-tabs/assets/styles.css";

function App() {
  const [activeId, setActiveId] = useState(1);
  const { sideBarPosition } = useSideBarStore();
  const { activityBarPosition } = useActivityBarStore();

  const renderPanel = () => {
    switch (activeId) {
      case 1:
        return <CollectionTreeView />;
      case 2:
        return <div>Environments Panel</div>;
      case 3:
        return <div>Mock Panel</div>;
      default:
        return null;
    }
  };

  return (
    <div className="background-(--moss-page-background) grid h-full grid-rows-[minmax(0px,46px)_1fr_auto] text-(--moss-text)">
      <HeadBar />
      <div className="flex h-full">
        <SideBar
          position={sideBarPosition}
          activityBarPosition={activityBarPosition}
          activeId={activeId}
          onSelect={setActiveId}
        >
          {renderPanel()}
        </SideBar>
        <Resizable>
          <ResizablePanel>
            <TabbedPane theme="dockview-theme-light" />
          </ResizablePanel>
        </Resizable>
      </div>
    </div>
  );
}

export default App;
