import { AllotmentHandle, LayoutPriority } from "allotment";
import { ReactNode, useEffect, useRef } from "react";

import { ActivityBar, BottomPane, Sidebar, SidebarEdgeHandler } from "@/components";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks";
import { useGetBottomPanel } from "@/hooks/sharedStorage/layout/bottomPanel/useGetBottomPanel";
import { useUpdateBottomPanel } from "@/hooks/sharedStorage/layout/bottomPanel/useUpdateBottomPanel";
import { useGetSidebarPanel } from "@/hooks/sharedStorage/layout/sidebar/useGetSidebarPanel";
import { useUpdateSidebarPanel } from "@/hooks/sharedStorage/layout/sidebar/useUpdateSidebarPanel";
import { useActivityBarStore } from "@/store/activityBar";

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

  const { data: sideBar } = useGetSidebarPanel();
  const { mutate: updateSidebarPanel } = useUpdateSidebarPanel();

  const { data: bottomPane } = useGetBottomPanel();
  const { mutate: updateBottomPanel } = useUpdateBottomPanel();

  const handleSidebarEdgeHandlerClick = () => {
    if (!sideBar?.visible && activeWorkspaceId) updateSidebarPanel({ visible: true, workspaceId: activeWorkspaceId });
  };

  const handleBottomPaneEdgeHandlerClick = () => {
    if (!bottomPane?.visible && activeWorkspaceId) updateBottomPanel({ visible: true, workspaceId: activeWorkspaceId });
  };

  useEffect(() => {
    verticalResizableRef?.current?.reset();
    mainResizableRef?.current?.reset();
    // We want to run this effect(resetting the layout) when the workspace changes, because different workspaces have different layouts.
  }, [activeWorkspaceId]);

  return (
    <div className="AppLayout flex h-full w-full">
      {position === ACTIVITYBAR_POSITION.DEFAULT && sideBar?.position === SIDEBAR_POSITION.LEFT && <ActivityBar />}
      <div className="relative flex h-full w-full">
        {!sideBar?.visible && sideBar?.position === SIDEBAR_POSITION.LEFT && (
          <SidebarEdgeHandler alignment="left" onClick={handleSidebarEdgeHandlerClick} />
        )}

        <Resizable
          ref={mainResizableRef}
          proportionalLayout={false}
          onDragEnd={(sizes) => {
            if (sideBar?.position === SIDEBAR_POSITION.LEFT) {
              const [leftPanelSize, _mainPanelSize] = sizes;
              updateSidebarPanel({ size: leftPanelSize, visible: leftPanelSize > 0, workspaceId: activeWorkspaceId });
            }
            if (sideBar?.position === SIDEBAR_POSITION.RIGHT) {
              const [_mainPanelSize, rightPanelSize] = sizes;
              updateSidebarPanel({ size: rightPanelSize, visible: rightPanelSize > 0, workspaceId: activeWorkspaceId });
            }
          }}
          onVisibleChange={(index, visible) => {
            if (sideBar?.position === SIDEBAR_POSITION.LEFT && index === 0)
              updateSidebarPanel({ visible, size: sideBar?.size ?? 0 });
            if (sideBar?.position === SIDEBAR_POSITION.RIGHT && index === 1)
              updateSidebarPanel({ visible, size: sideBar?.size ?? 0 });
          }}
        >
          {sideBar?.position === SIDEBAR_POSITION.LEFT && (
            <ResizablePanel
              preferredSize={sideBar?.size}
              visible={sideBar?.visible && sideBar?.position === SIDEBAR_POSITION.LEFT}
              minSize={sideBar?.minWidth}
              maxSize={sideBar?.maxWidth}
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
                if (activeWorkspaceId) updateBottomPanel({ height: bottomPaneSize });
              }}
              onVisibleChange={(_, visible) => {
                if (activeWorkspaceId) updateBottomPanel({ visible });
              }}
            >
              <ResizablePanel>{children ?? <MainContent />}</ResizablePanel>
              <ResizablePanel
                preferredSize={bottomPane?.height}
                visible={bottomPane?.visible}
                minSize={bottomPane?.minHeight}
                snap
              >
                <BottomPaneContent />
              </ResizablePanel>
            </Resizable>
            {!bottomPane?.visible && (
              <SidebarEdgeHandler alignment="bottom" onClick={handleBottomPaneEdgeHandlerClick} />
            )}
          </ResizablePanel>

          {sideBar?.position === SIDEBAR_POSITION.RIGHT && (
            <ResizablePanel
              preferredSize={sideBar?.size}
              visible={sideBar?.visible && sideBar?.position === SIDEBAR_POSITION.RIGHT}
              minSize={sideBar?.minWidth}
              maxSize={sideBar?.maxWidth}
              snap
              className="background-(--moss-primary-background)"
            >
              <SidebarContent />
            </ResizablePanel>
          )}
        </Resizable>

        {!sideBar?.visible && sideBar?.position === SIDEBAR_POSITION.RIGHT && (
          <SidebarEdgeHandler alignment="right" onClick={handleSidebarEdgeHandlerClick} />
        )}
      </div>

      {position === ACTIVITYBAR_POSITION.DEFAULT && sideBar?.position === SIDEBAR_POSITION.RIGHT && <ActivityBar />}
    </div>
  );
};

const SidebarContent = () => <Sidebar />;

const MainContent = () => <TabbedPane />;

const BottomPaneContent = () => <BottomPane />;
