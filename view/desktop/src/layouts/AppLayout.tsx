import { AllotmentHandle, LayoutPriority } from "allotment";
import { ReactNode, useEffect, useRef } from "react";

import { ActivityBar, BottomPane, Sidebar, SidebarEdgeHandler } from "@/components";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import { Resizable, ResizablePanel } from "../lib/ui/Resizable";
import TabbedPane from "../parts/TabbedPane/TabbedPane";

interface AppLayoutProps {
  children?: ReactNode;
}

export const AppLayout = ({ children }: AppLayoutProps) => {
  const mainResizableRef = useRef<AllotmentHandle>(null);
  const verticalResizableRef = useRef<AllotmentHandle>(null);

  const { activeWorkspaceId } = useActiveWorkspace();

  const { position } = useActivityBarStore();
  const { bottomPane, sideBar, sideBarPosition, initialize } = useAppResizableLayoutStore();

  const handleSidebarEdgeHandlerClick = () => {
    if (!sideBar.visible && activeWorkspaceId) sideBar.setVisible(true, activeWorkspaceId);
  };

  const handleBottomPaneEdgeHandlerClick = () => {
    if (!bottomPane.visible && activeWorkspaceId) bottomPane.setVisible(true, activeWorkspaceId);
  };

  useEffect(() => {
    const resetLayout = async () => {
      if (!mainResizableRef.current || !verticalResizableRef.current) return;

      if (activeWorkspaceId) await initialize(activeWorkspaceId);

      verticalResizableRef.current.reset();
      mainResizableRef.current.reset();
    };
    resetLayout();
    // We only want to run this effect when the active workspace changes, to reset the layout, because different workspaces have different layouts.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [activeWorkspaceId]);

  return (
    <div className="AppLayout flex h-full w-full">
      {position === ACTIVITYBAR_POSITION.DEFAULT && sideBarPosition === SIDEBAR_POSITION.LEFT && <ActivityBar />}
      <div className="relative flex h-full w-full">
        {!sideBar.visible && sideBarPosition === SIDEBAR_POSITION.LEFT && (
          <SidebarEdgeHandler alignment="left" onClick={handleSidebarEdgeHandlerClick} />
        )}

        <Resizable
          ref={mainResizableRef}
          proportionalLayout={false}
          onDragEnd={(sizes) => {
            if (sideBarPosition === SIDEBAR_POSITION.LEFT) {
              const [leftPanelSize, _mainPanelSize] = sizes;
              if (activeWorkspaceId) sideBar.setWidth(leftPanelSize, activeWorkspaceId);
            }
            if (sideBarPosition === SIDEBAR_POSITION.RIGHT) {
              const [_mainPanelSize, rightPanelSize] = sizes;
              if (activeWorkspaceId) sideBar.setWidth(rightPanelSize, activeWorkspaceId);
            }
          }}
          onVisibleChange={(index, visible) => {
            if (sideBarPosition === SIDEBAR_POSITION.LEFT && index === 0)
              if (activeWorkspaceId) sideBar.setVisible(visible, activeWorkspaceId);
            if (sideBarPosition === SIDEBAR_POSITION.RIGHT && index === 1)
              if (activeWorkspaceId) sideBar.setVisible(visible, activeWorkspaceId);
          }}
        >
          {sideBarPosition === SIDEBAR_POSITION.LEFT && (
            <ResizablePanel
              preferredSize={sideBar.width}
              visible={sideBar.visible && sideBarPosition === SIDEBAR_POSITION.LEFT}
              minSize={sideBar.minWidth}
              maxSize={sideBar.maxWidth}
              snap
              className="background-(--moss-primary-background)"
            >
              <SidebarContent />
            </ResizablePanel>
          )}
          <ResizablePanel priority={LayoutPriority.High}>
            <Resizable
              ref={verticalResizableRef}
              className="relative"
              vertical
              onDragEnd={(sizes) => {
                const [_mainPanelSize, bottomPaneSize] = sizes;
                if (activeWorkspaceId) bottomPane.setHeight(bottomPaneSize, activeWorkspaceId);
              }}
              onVisibleChange={(index, visible) => {
                if (activeWorkspaceId) bottomPane.setVisible(visible, activeWorkspaceId);
              }}
            >
              <ResizablePanel>{children ?? <MainContent />}</ResizablePanel>
              <ResizablePanel
                preferredSize={bottomPane.height}
                visible={bottomPane.visible}
                minSize={bottomPane.minHeight}
                snap
              >
                <BottomPaneContent />
              </ResizablePanel>
            </Resizable>
            {!bottomPane.visible && (
              <SidebarEdgeHandler alignment="bottom" onClick={handleBottomPaneEdgeHandlerClick} />
            )}
          </ResizablePanel>

          {sideBarPosition === SIDEBAR_POSITION.RIGHT && (
            <ResizablePanel
              preferredSize={sideBar.width}
              visible={sideBar.visible && sideBarPosition === SIDEBAR_POSITION.RIGHT}
              minSize={sideBar.minWidth}
              maxSize={sideBar.maxWidth}
              snap
              className="background-(--moss-primary-background)"
            >
              <SidebarContent />
            </ResizablePanel>
          )}
        </Resizable>

        {!sideBar.visible && sideBarPosition === SIDEBAR_POSITION.RIGHT && (
          <SidebarEdgeHandler alignment="right" onClick={handleSidebarEdgeHandlerClick} />
        )}
      </div>

      {position === ACTIVITYBAR_POSITION.DEFAULT && sideBarPosition === SIDEBAR_POSITION.RIGHT && <ActivityBar />}
    </div>
  );
};

const SidebarContent = () => <Sidebar />;

const MainContent = () => <TabbedPane />;

const BottomPaneContent = () => <BottomPane />;
