import { AllotmentHandle, LayoutPriority } from "allotment";
import { ReactNode, useEffect, useRef } from "react";

import { ActivityBar, BottomPane, Sidebar, SidebarEdgeHandler } from "@/components";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks";
import { useDescribeWorkspaceState } from "@/hooks/workspace/useDescribeWorkspaceState";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import { Resizable, ResizablePanel } from "../lib/ui/Resizable";
import TabbedPane from "../parts/TabbedPane/TabbedPane";

interface AppLayoutProps {
  children?: ReactNode;
}

export const AppLayout = ({ children }: AppLayoutProps) => {
  const resizableRef = useRef<AllotmentHandle>(null);

  const { data: workspaceState } = useDescribeWorkspaceState();
  const { activeWorkspaceId } = useActiveWorkspace();

  const { position } = useActivityBarStore();
  const { bottomPane, sideBar, sideBarPosition, initialize } = useAppResizableLayoutStore();

  const handleSidebarEdgeHandlerClick = () => {
    if (!sideBar.visible) sideBar.setVisible(true);
  };

  const handleBottomPaneEdgeHandlerClick = () => {
    if (!bottomPane.visible) bottomPane.setVisible(true);
  };

  useEffect(() => {
    if (!resizableRef.current) return;

    initialize({
      sideBar: {
        width: workspaceState?.layouts.sidebar?.size ?? 255,
        visible: workspaceState?.layouts.sidebar?.visible ?? true,
      },
      bottomPane: {
        height: workspaceState?.layouts.panel?.size ?? 255,
        visible: workspaceState?.layouts.panel?.visible ?? true,
      },
    });

    resizableRef.current.reset();
    // We only want to run this effect when the active workspace changes, to reset the layout, because different workspaces have different layouts.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [activeWorkspaceId]);

  return (
    <div className="flex h-full w-full">
      {position === ACTIVITYBAR_POSITION.DEFAULT && sideBarPosition === SIDEBAR_POSITION.LEFT && <ActivityBar />}
      <div className="relative flex h-full w-full">
        {!sideBar.visible && sideBarPosition === SIDEBAR_POSITION.LEFT && (
          <SidebarEdgeHandler alignment="left" onClick={handleSidebarEdgeHandlerClick} />
        )}

        <Resizable
          proportionalLayout={false}
          ref={resizableRef}
          onDragEnd={(sizes) => {
            if (sideBarPosition === SIDEBAR_POSITION.LEFT) {
              const [leftPanelSize, _mainPanelSize] = sizes;
              sideBar.setWidth(leftPanelSize);
            }
            if (sideBarPosition === SIDEBAR_POSITION.RIGHT) {
              const [_mainPanelSize, rightPanelSize] = sizes;
              sideBar.setWidth(rightPanelSize);
            }
          }}
          onVisibleChange={(index, visible) => {
            if (sideBarPosition === SIDEBAR_POSITION.LEFT && index === 0) sideBar.setVisible(visible);
            if (sideBarPosition === SIDEBAR_POSITION.RIGHT && index === 1) sideBar.setVisible(visible);
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
              className="relative"
              ref={resizableRef}
              vertical
              onDragEnd={(sizes) => {
                const [_mainPanelSize, bottomPaneSize] = sizes;
                bottomPane.setHeight(bottomPaneSize);
              }}
              onVisibleChange={(index, visible) => {
                if (index === 0) bottomPane.setVisible(visible);
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
