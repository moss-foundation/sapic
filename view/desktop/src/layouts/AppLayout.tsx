import { AllotmentHandle, LayoutPriority } from "allotment";
import { ReactNode, useEffect, useRef } from "react";

import { ActivityBar, BottomPane, Sidebar, SidebarEdgeHandler } from "@/components";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutStates";
import { useActiveWorkspace, useDescribeApp } from "@/hooks";
import { useGetLayout } from "@/hooks/sharedStorage/layout/useGetLayout";
import { useUpdateLayout } from "@/hooks/sharedStorage/layout/useUpdateLayout";

import { Resizable, ResizablePanel } from "../lib/ui/Resizable";
import TabbedPane from "../parts/TabbedPane/TabbedPane";

interface AppLayoutProps {
  children?: ReactNode;
}

export const AppLayout = ({ children }: AppLayoutProps) => {
  const mainResizableRef = useRef<AllotmentHandle>(null);
  const verticalResizableRef = useRef<AllotmentHandle>(null);

  const { data: appState } = useDescribeApp();
  const { activeWorkspaceId } = useActiveWorkspace();

  const { data: layout } = useGetLayout();
  const { mutate: updateLayout } = useUpdateLayout();

  const activityBarPosition = appState?.configuration.contents.activityBarPosition || ACTIVITYBAR_POSITION.DEFAULT;
  const sideBarPosition = appState?.configuration.contents.sideBarPosition || SIDEBAR_POSITION.LEFT;

  useEffect(() => {
    verticalResizableRef?.current?.reset();
    mainResizableRef?.current?.reset();
    // We want to run this effect(resetting the layout) when the workspace changes
    // because different workspaces have different layouts.
  }, [activeWorkspaceId]);

  const handleSidebarEdgeHandlerClick = () => {
    if (!layout?.sidebarState.visible && activeWorkspaceId) {
      updateLayout({
        layout: {
          sidebarState: {
            visible: true,
          },
        },
        workspaceId: activeWorkspaceId,
      });
    }
  };

  const handleBottomPaneEdgeHandlerClick = () => {
    if (!layout?.bottomPanelState.visible && activeWorkspaceId) {
      updateLayout({
        layout: {
          bottomPanelState: {
            visible: true,
          },
        },
        workspaceId: activeWorkspaceId,
      });
    }
  };

  return (
    <div className="AppLayout flex h-full w-full">
      {activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT && sideBarPosition === SIDEBAR_POSITION.LEFT && (
        <ActivityBar />
      )}
      <div className="relative flex h-full w-full">
        {/* FIXME: we can hide the sidebar when out of workspace, but cannot shot it back */}
        {!layout?.sidebarState.visible && sideBarPosition === SIDEBAR_POSITION.LEFT && (
          <SidebarEdgeHandler alignment="left" onClick={handleSidebarEdgeHandlerClick} />
        )}

        <Resizable
          ref={mainResizableRef}
          proportionalLayout={false}
          onDragEnd={(sizes) => {
            if (!activeWorkspaceId) return;

            if (sideBarPosition === SIDEBAR_POSITION.LEFT) {
              const [leftPanelSize, _mainPanelSize] = sizes;
              if (leftPanelSize <= 0) {
                updateLayout({
                  layout: { sidebarState: { visible: false } },
                  workspaceId: activeWorkspaceId,
                });
              } else {
                updateLayout({
                  layout: { sidebarState: { width: leftPanelSize } },
                  workspaceId: activeWorkspaceId,
                });
              }
            }
            if (sideBarPosition === SIDEBAR_POSITION.RIGHT) {
              const [_mainPanelSize, rightPanelSize] = sizes;
              if (rightPanelSize <= 0) {
                updateLayout({
                  layout: { sidebarState: { visible: false } },
                  workspaceId: activeWorkspaceId,
                });
              } else {
                updateLayout({
                  layout: { sidebarState: { width: rightPanelSize } },
                  workspaceId: activeWorkspaceId,
                });
              }
            }
          }}
          onVisibleChange={(index, visible) => {
            if (!activeWorkspaceId) return;

            if (sideBarPosition === SIDEBAR_POSITION.LEFT && index === 0) {
              updateLayout({
                layout: { sidebarState: { visible: visible } },
                workspaceId: activeWorkspaceId,
              });
            }
            if (sideBarPosition === SIDEBAR_POSITION.RIGHT && index === 1) {
              updateLayout({
                layout: { sidebarState: { visible: visible } },
                workspaceId: activeWorkspaceId,
              });
            }
          }}
        >
          {sideBarPosition === SIDEBAR_POSITION.LEFT && (
            <ResizablePanel
              preferredSize={layout?.sidebarState.width}
              visible={layout?.sidebarState.visible && sideBarPosition === SIDEBAR_POSITION.LEFT}
              minSize={layout?.sidebarState.minWidth}
              maxSize={layout?.sidebarState.maxWidth}
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
                if (!activeWorkspaceId) return;

                const [_mainPanelSize, bottomPaneSize] = sizes;

                if (bottomPaneSize <= 0) {
                  updateLayout({
                    layout: { bottomPanelState: { visible: false } },
                    workspaceId: activeWorkspaceId,
                  });
                } else {
                  updateLayout({
                    layout: { bottomPanelState: { height: bottomPaneSize } },
                    workspaceId: activeWorkspaceId,
                  });
                }
              }}
              onVisibleChange={(_, visible) => {
                if (!activeWorkspaceId) return;

                updateLayout({
                  layout: {
                    bottomPanelState: { visible },
                  },
                  workspaceId: activeWorkspaceId,
                });
              }}
            >
              <ResizablePanel>{children ?? <MainContent />}</ResizablePanel>
              <ResizablePanel
                preferredSize={layout?.bottomPanelState.height}
                visible={layout?.bottomPanelState.visible}
                minSize={layout?.bottomPanelState.minHeight}
                snap
              >
                <BottomPaneContent />
              </ResizablePanel>
            </Resizable>
            {!layout?.bottomPanelState.visible && (
              <SidebarEdgeHandler alignment="bottom" onClick={handleBottomPaneEdgeHandlerClick} />
            )}
          </ResizablePanel>

          {sideBarPosition === SIDEBAR_POSITION.RIGHT && (
            <ResizablePanel
              preferredSize={layout?.sidebarState.width}
              visible={layout?.sidebarState.visible && sideBarPosition === SIDEBAR_POSITION.RIGHT}
              minSize={layout?.sidebarState.minWidth}
              maxSize={layout?.sidebarState.maxWidth}
              snap
              className="background-(--moss-primary-background)"
            >
              <SidebarContent />
            </ResizablePanel>
          )}
        </Resizable>

        {!layout?.sidebarState.visible && sideBarPosition === SIDEBAR_POSITION.RIGHT && (
          <SidebarEdgeHandler alignment="right" onClick={handleSidebarEdgeHandlerClick} />
        )}
      </div>

      {activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT && sideBarPosition === SIDEBAR_POSITION.RIGHT && (
        <ActivityBar />
      )}
    </div>
  );
};

const SidebarContent = () => <Sidebar />;

const MainContent = () => <TabbedPane />;

const BottomPaneContent = () => <BottomPane />;
