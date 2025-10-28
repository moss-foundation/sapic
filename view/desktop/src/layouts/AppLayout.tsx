import { AllotmentHandle, LayoutPriority } from "allotment";
import { ReactNode, useEffect, useRef, useState } from "react";

import { ActivityBar, BottomPane, Sidebar } from "@/components";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useUpdateActivitybarPartState } from "@/hooks/app/useUpdateActivitybarPartState";
import { useUpdatePanelPartState } from "@/hooks/app/useUpdatePanelPartState";
import { useActiveWorkspace } from "@/hooks/workspace/derived/useActiveWorkspace";
import { useDescribeWorkspaceState } from "@/hooks/workspace/useDescribeWorkspaceState";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";

import { Resizable, ResizablePanel } from "../lib/ui/Resizable";
import TabbedPane from "../parts/TabbedPane/TabbedPane";

interface AppLayoutProps {
  children?: ReactNode;
}

export const AppLayout = ({ children }: AppLayoutProps) => {
  const { activeWorkspaceId, hasActiveWorkspace } = useActiveWorkspace();
  const canUpdatePartState = useRef(false);
  const lastProcessedWorkspaceId = useRef<string | null>(null);
  const isTransitioning = useRef(false);

  const { position, toWorkspaceState } = useActivityBarStore();
  const { bottomPane, sideBar, sideBarPosition } = useAppResizableLayoutStore();

  // Fetch workspace state to know when initialization is complete
  const { data: workspaceState, isFetched, isSuccess } = useDescribeWorkspaceState();

  // Reset update permission when workspace changes
  useEffect(() => {
    if (lastProcessedWorkspaceId.current !== activeWorkspaceId) {
      canUpdatePartState.current = false;
      isTransitioning.current = true;
      lastProcessedWorkspaceId.current = activeWorkspaceId;
    }
  }, [activeWorkspaceId]);

  // Initialize bottom pane state from workspace data (panels are still managed here)
  useEffect(() => {
    if (hasActiveWorkspace && (!isFetched || !isSuccess)) {
      // Still waiting for workspace state
      canUpdatePartState.current = false;
      isTransitioning.current = true;
      return;
    }

    // Initialize bottom pane from workspace state if available
    if (workspaceState?.layouts.panel) {
      const { initialize } = useAppResizableLayoutStore.getState();
      initialize({
        sideBar: {
          // Don't modify sidebar - handled by workspace hooks
        },
        bottomPane: {
          height: workspaceState.layouts.panel.size,
          visible: workspaceState.layouts.panel.visible,
        },
      });
    }

    // Enable updates after workspace state is loaded (or when no workspace)
    // Use a delay to ensure all initialization effects have run
    setTimeout(() => {
      canUpdatePartState.current = true;
      isTransitioning.current = false;
    }, 200);
  }, [hasActiveWorkspace, workspaceState, isFetched, isSuccess]);

  const handleSidebarEdgeHandlerClick = () => {
    if (!sideBar.visible) sideBar.setVisible(true);
  };

  const handleBottomPaneEdgeHandlerClick = () => {
    if (!bottomPane.visible) bottomPane.setVisible(true);
  };

  const resizableRef = useRef<AllotmentHandle>(null);

  useEffect(() => {
    if (!resizableRef.current) return;

    resizableRef.current.reset();
  }, [bottomPane, sideBar, sideBarPosition]);

  const { mutate: updatePanelPartState } = useUpdatePanelPartState();
  useEffect(() => {
    if (!canUpdatePartState.current || !activeWorkspaceId || isTransitioning.current) return;

    updatePanelPartState({
      size: bottomPane.height,
      visible: bottomPane.visible,
    });
  }, [activeWorkspaceId, bottomPane, updatePanelPartState]);

  // ActivityBar state persistence - only save when workspace is stable and initialization is complete
  const { mutate: updateActivitybarPartState } = useUpdateActivitybarPartState();
  const activityBarState = useActivityBarStore();
  useEffect(() => {
    if (!canUpdatePartState.current || !activeWorkspaceId || isTransitioning.current) return;

    updateActivitybarPartState(toWorkspaceState());
  }, [
    activeWorkspaceId,
    activityBarState.position,
    activityBarState.items,
    activityBarState.lastActiveContainerId,
    updateActivitybarPartState,
    toWorkspaceState,
  ]);

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

const BottomPaneContent = () => {
  return <BottomPane />;
};

interface SidebarEdgeHandlerProps {
  alignment?: "left" | "right" | "bottom";
  onClick?: () => void;
}

const SidebarEdgeHandler = ({ alignment, onClick }: SidebarEdgeHandlerProps) => {
  const [showBg, setShowBg] = useState(false);
  return (
    <div
      className={cn("absolute z-40", {
        "left-0 h-full w-2": alignment === "left",
        "right-0 h-full w-2": alignment === "right",
        "bottom-0 h-2 w-full": alignment === "bottom",
      })}
    >
      {/* handle bg*/}
      <div
        className={cn(`background-(--moss-accent)/50 absolute z-40 hidden cursor-pointer`, {
          "left-0 top-0 h-full w-3": alignment === "left",
          "right-0 top-0 h-full w-3": alignment === "right",
          "bottom-0 left-0 h-3 w-full": alignment === "bottom",
          "block": showBg,
        })}
        onMouseEnter={() => setShowBg(true)}
        onMouseLeave={() => setShowBg(false)}
        onClick={onClick}
      />

      {/* handle */}
      <div
        className={cn(
          `background-(--moss-accent)/50 hover:background-(--moss-accent)/80 absolute z-50 cursor-pointer rounded`,
          {
            "inset-y-[calc(50%-64px)] left-[3px] h-32 w-1.5": alignment === "left",
            "inset-y-[calc(50%-64px)] right-[3px] h-32 w-1.5": alignment === "right",
            "inset-x-[calc(50%-64px)] bottom-[3px] h-1.5 w-32": alignment === "bottom",
            "background-(--moss-accent)/80": showBg,
          }
        )}
        onMouseEnter={() => setShowBg(true)}
        onClick={onClick}
      />
    </div>
  );
};
