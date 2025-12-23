import { AllotmentHandle, LayoutPriority } from "allotment";
import { useEffect, useRef } from "react";

import { useActiveWorkspace } from "@/hooks";
import { useGetLayout, useUpdateLayout } from "@/workbench/adapters";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/workbench/domains/layout";
import { ActivityBar, SidebarEdgeHandler } from "@/workbench/ui/components";
import { BottomPane, Sidebar, TabbedPane } from "@/workbench/ui/parts";

import { Resizable, ResizablePanel } from "../../lib/ui/Resizable";

export const AppLayout = () => {
  const mainResizableRef = useRef<AllotmentHandle>(null);
  const verticalResizableRef = useRef<AllotmentHandle>(null);

  const { activeWorkspaceId } = useActiveWorkspace();

  const { data: layout } = useGetLayout();
  const { mutate: updateLayout } = useUpdateLayout();

  //TODO later we should handle the JsonValue differently
  const activityBarPosition = layout?.activitybarState.position || ACTIVITYBAR_POSITION.DEFAULT;
  const sideBarPosition = layout?.sidebarState.position || SIDEBAR_POSITION.LEFT;

  useEffect(() => {
    verticalResizableRef?.current?.reset();
    mainResizableRef?.current?.reset();
    // We want to run this effect(resetting the layout) when the workspace changes
    // because different workspaces have different layouts.
  }, [activeWorkspaceId, layout]);

  const handleSidebarEdgeHandlerClick = () => {
    if (!activeWorkspaceId) return;

    if (!layout?.sidebarState.visible) {
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
    if (!activeWorkspaceId) return;

    if (!layout?.bottomPanelState.visible) {
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

  if (!activeWorkspaceId) return null;

  return (
    <div className="flex h-full w-full">
      {activityBarPosition === ACTIVITYBAR_POSITION.DEFAULT && sideBarPosition === SIDEBAR_POSITION.LEFT && (
        <ActivityBar />
      )}
      <div className="relative flex h-full w-full">
        {!layout?.sidebarState.visible && sideBarPosition === SIDEBAR_POSITION.LEFT && (
          <SidebarEdgeHandler alignment="left" onClick={handleSidebarEdgeHandlerClick} />
        )}

        <Resizable
          ref={mainResizableRef}
          proportionalLayout={false}
          onDragEnd={(sizes) => {
            const [leftPanelSize, rightPanelSize] = sizes;
            const updatedWidth = sideBarPosition === SIDEBAR_POSITION.LEFT ? leftPanelSize : rightPanelSize;

            if (updatedWidth <= 0) {
              updateLayout({
                layout: { sidebarState: { visible: false } },
                workspaceId: activeWorkspaceId,
              });
            } else {
              updateLayout({
                layout: { sidebarState: { width: updatedWidth } },
                workspaceId: activeWorkspaceId,
              });
            }
          }}
          onVisibleChange={(index, visible) => {
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
                updateLayout({
                  layout: {
                    bottomPanelState: { visible },
                  },
                  workspaceId: activeWorkspaceId,
                });
              }}
            >
              <ResizablePanel>
                <MainContent />
              </ResizablePanel>
              <ResizablePanel
                preferredSize={layout?.bottomPanelState.height}
                visible={layout?.bottomPanelState.visible}
                minSize={layout?.bottomPanelState.minHeight}
                maxSize={layout?.bottomPanelState.maxHeight}
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
