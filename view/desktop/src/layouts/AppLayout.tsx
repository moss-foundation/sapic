import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import "@repo/moss-tabs/assets/styles.css";

import { AllotmentHandle, LayoutPriority } from "allotment";
import { ReactNode, useEffect, useRef, useState } from "react";

import { ActivityBar, BottomPane, Sidebar } from "@/components";
import { useUpdatePanelPartState } from "@/hooks/appState/useUpdatePanelPartState";
import { useUpdateActivitybarPartState } from "@/hooks/appState/useUpdateActivitybarPartState";
import { useActivityBarStore } from "@/store/activityBar";
import { useActiveWorkspace } from "@/hooks/workspace/useActiveWorkspace";
import { useDescribeWorkspaceState } from "@/hooks/workspace/useDescribeWorkspaceState";
import { cn } from "@/utils";

import { Resizable, ResizablePanel } from "../lib/ui/Resizable";
import TabbedPane from "../parts/TabbedPane/TabbedPane";

interface AppLayoutProps {
  children?: ReactNode;
}

export const AppLayout = ({ children }: AppLayoutProps) => {
  const workspace = useActiveWorkspace();
  const canUpdatePartState = useRef(false);

  const { position, toWorkspaceState } = useActivityBarStore();
  const { bottomPane, sideBar, sideBarPosition } = useAppResizableLayoutStore();

  // Fetch workspace state to know when initialization is complete
  const { data: workspaceState, isFetched } = useDescribeWorkspaceState({
    enabled: !!workspace,
  });

  // Initialize bottom pane state from workspace data (panels are still managed here)
  useEffect(() => {
    if (workspace && !isFetched) {
      // Still waiting for workspace state
      canUpdatePartState.current = false;
      return;
    }

    // Initialize bottom pane from workspace state if available
    if (workspaceState?.panel) {
      const { initialize } = useAppResizableLayoutStore.getState();
      initialize({
        sideBar: {
          // Don't modify sidebar - handled by workspace hooks
        },
        bottomPane: {
          height: workspaceState.panel.size,
          visible: workspaceState.panel.visible,
        },
      });
    }

    // Enable updates after workspace state is loaded (or when no workspace)
    // Use a small delay to ensure all initialization effects have run
    setTimeout(() => {
      canUpdatePartState.current = true;
    }, 50);
  }, [workspace, workspaceState, isFetched]);

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
    if (!canUpdatePartState.current) return;

    updatePanelPartState({
      size: bottomPane.height,
      visible: bottomPane.visible,
    });
  }, [bottomPane, updatePanelPartState]);

  // ActivityBar state persistence
  const { mutate: updateActivitybarPartState } = useUpdateActivitybarPartState();
  const activityBarState = useActivityBarStore();
  useEffect(() => {
    if (!canUpdatePartState.current) return;

    updateActivitybarPartState(toWorkspaceState());
  }, [
    activityBarState.position,
    activityBarState.items,
    activityBarState.lastActiveContainerId,
    updateActivitybarPartState,
    toWorkspaceState,
  ]);

  return (
    <div className="flex h-full w-full">
      {position === "DEFAULT" && sideBarPosition === "LEFT" && <ActivityBar />}
      <div className="relative flex h-full w-full">
        {!sideBar.visible && sideBarPosition === "LEFT" && (
          <SidebarEdgeHandler alignment="left" onClick={handleSidebarEdgeHandlerClick} />
        )}

        <Resizable
          proportionalLayout={false}
          ref={resizableRef}
          onDragEnd={(sizes) => {
            if (sideBarPosition === "LEFT") {
              const [leftPanelSize, _mainPanelSize] = sizes;
              sideBar.setWidth(leftPanelSize);
            }
            if (sideBarPosition === "RIGHT") {
              const [_mainPanelSize, rightPanelSize] = sizes;
              sideBar.setWidth(rightPanelSize);
            }
          }}
          onVisibleChange={(index, visible) => {
            if (sideBarPosition === "LEFT" && index === 0) sideBar.setVisible(visible);
            if (sideBarPosition === "RIGHT" && index === 1) sideBar.setVisible(visible);
          }}
        >
          {sideBarPosition === "LEFT" && (
            <ResizablePanel
              preferredSize={sideBar.width}
              visible={sideBar.visible && sideBarPosition === "LEFT"}
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

          {sideBarPosition === "RIGHT" && (
            <ResizablePanel
              preferredSize={sideBar.width}
              visible={sideBar.visible && sideBarPosition === "RIGHT"}
              minSize={sideBar.minWidth}
              maxSize={sideBar.maxWidth}
              snap
              className="background-(--moss-primary-background)"
            >
              <SidebarContent />
            </ResizablePanel>
          )}
        </Resizable>

        {!sideBar.visible && sideBarPosition === "RIGHT" && (
          <SidebarEdgeHandler alignment="right" onClick={handleSidebarEdgeHandlerClick} />
        )}
      </div>

      {position === "DEFAULT" && sideBarPosition === "RIGHT" && <ActivityBar />}
    </div>
  );
};

const SidebarContent = () => <Sidebar />;

const MainContent = () => <TabbedPane theme="dockview-theme-light" />;

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
        className={cn(`background-(--moss-info-background-hover)/70 absolute z-40 hidden cursor-pointer`, {
          "top-0 left-0 h-full w-3": alignment === "left",
          "top-0 right-0 h-full w-3": alignment === "right",
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
          `background-(--moss-primary)/50 hover:background-(--moss-primary)/80 absolute z-50 cursor-pointer rounded`,
          {
            "inset-y-[calc(50%-64px)] left-[3px] h-32 w-1.5": alignment === "left",
            "inset-y-[calc(50%-64px)] right-[3px] h-32 w-1.5": alignment === "right",
            "inset-x-[calc(50%-64px)] bottom-[3px] h-1.5 w-32": alignment === "bottom",
            "background-(--moss-info-icon)/80": showBg,
          }
        )}
        onMouseEnter={() => setShowBg(true)}
        onClick={onClick}
      />
    </div>
  );
};
