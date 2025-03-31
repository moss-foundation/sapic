import { LayoutPriority } from "allotment";
import { Suspense, useState, useEffect, useRef } from "react";

import { useGetAppLayoutState } from "@/hooks/useGetAppLayoutState";
import { useChangeAppLayoutState } from "@/hooks/useChangeAppLayoutState";
import { useGetActivityBarState } from "@/hooks/useGetActivityBarState";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import {
  ACTIVITY_BAR_WIDTH,
  DEFAULT_SIDEBAR_WIDTH,
  MIN_SIDEBAR_WIDTH,
  MAX_SIDEBAR_WIDTH,
  MIN_BOTTOM_PANE_HEIGHT,
  MIN_BOTTOM_PANE_DRAGGABLE_HEIGHT,
  DEFAULT_BOTTOM_PANE_HEIGHT,
  SIDEBAR_COLLAPSE_THRESHOLD,
  BOTTOM_PANE_COLLAPSE_THRESHOLD,
  SIDEBAR_POSITION_LEFT,
  SIDEBAR_POSITION_RIGHT,
  SIDEBAR_POSITION_NONE,
} from "@/constants/layout";

import "@repo/moss-tabs/assets/styles.css";

import { Sidebar } from "@/components";
import { VerticalActivityBar } from "@/parts/ActivityBar/VerticalActivityBar";
import { SidebarEdgeHandler } from "@/parts/SideBar/SidebarEdgeHandler";
import { BottomPane } from "@/parts/BottomPane/BottomPane";
import { BottomPaneEdgeHandler } from "@/parts/BottomPane/BottomPaneEdgeHandler";

import { Resizable, ResizablePanel } from "../components/Resizable";
import TabbedPane from "../parts/TabbedPane/TabbedPane";
import { ContentLayout } from "./ContentLayout";

export const AppLayout = () => {
  const { data: appLayoutState } = useGetAppLayoutState();
  const { mutate: changeAppLayoutState } = useChangeAppLayoutState();
  const { data: activityBarState } = useGetActivityBarState();

  const sideBarSetWidth = useAppResizableLayoutStore((state) => state.sideBar.setWidth);
  const sideBarGetWidth = useAppResizableLayoutStore((state) => state.sideBar.getWidth);
  const lastSidebarWidthRef = useRef(sideBarGetWidth() || DEFAULT_SIDEBAR_WIDTH);

  const [isResizing, setIsResizing] = useState(false);

  const bottomPaneVisibility = useAppResizableLayoutStore((state) => state.bottomPane.visibility);
  const bottomPaneSetHeight = useAppResizableLayoutStore((state) => state.bottomPane.setHeight);
  const bottomPaneGetHeight = useAppResizableLayoutStore((state) => state.bottomPane.getHeight);
  const bottomPaneSetVisibility = useAppResizableLayoutStore((state) => state.bottomPane.setVisibility);
  const lastBottomPaneHeightRef = useRef(bottomPaneGetHeight() || MIN_BOTTOM_PANE_HEIGHT);

  const sidebarVisible = appLayoutState?.activeSidebar !== SIDEBAR_POSITION_NONE;
  const sidebarSide = appLayoutState?.sidebarSetting || SIDEBAR_POSITION_LEFT;
  const isLeftSidebar = sidebarSide === SIDEBAR_POSITION_LEFT;
  const isRightSidebar = sidebarSide === SIDEBAR_POSITION_RIGHT;

  const isActivityBarDefault = activityBarState?.position === "default";
  const shouldRenderStandaloneActivityBar = isActivityBarDefault;
  const shouldShowSidebar = sidebarVisible;

  useEffect(() => {
    if (sidebarVisible && sideBarGetWidth() > MIN_SIDEBAR_WIDTH) {
      lastSidebarWidthRef.current = sideBarGetWidth();
    }
  }, [sidebarVisible, sideBarGetWidth]);

  useEffect(() => {
    // Only save the height if the pane is visible and the height is reasonable
    if (bottomPaneVisibility && bottomPaneGetHeight() >= MIN_BOTTOM_PANE_HEIGHT) {
      lastBottomPaneHeightRef.current = bottomPaneGetHeight();
    }
  }, [bottomPaneVisibility, bottomPaneGetHeight]);

  const handleShowSidebar = () => {
    changeAppLayoutState({
      activeSidebar: sidebarSide,
      sidebarSetting: sidebarSide,
    });

    sideBarSetWidth(lastSidebarWidthRef.current);
  };

  const handleShowBottomPane = () => {
    bottomPaneSetVisibility(true);
    bottomPaneSetHeight(DEFAULT_BOTTOM_PANE_HEIGHT);
  };

  return (
    <div className="relative h-full w-full">
      {/* Standalone VerticalActivityBar when in default position */}
      {shouldRenderStandaloneActivityBar && <VerticalActivityBar position={sidebarSide} />}

      <div
        className="relative h-full w-full"
        style={{
          paddingLeft: shouldRenderStandaloneActivityBar && isLeftSidebar ? ACTIVITY_BAR_WIDTH : 0,
          paddingRight: shouldRenderStandaloneActivityBar && isRightSidebar ? ACTIVITY_BAR_WIDTH : 0,
        }}
      >
        {/* Edge handlers - visible only when sidebar is hidden */}
        {!sidebarVisible && (
          <SidebarEdgeHandler
            position={sidebarSide}
            onClick={handleShowSidebar}
            activityBarOffset={shouldRenderStandaloneActivityBar ? ACTIVITY_BAR_WIDTH : 0}
          />
        )}

        <Resizable
          onDragStart={() => setIsResizing(true)}
          onDragEnd={(sizes) => {
            setIsResizing(false);
            if (sidebarVisible) {
              if (isLeftSidebar) {
                const leftWidth = sizes[0];
                sideBarSetWidth(leftWidth);

                // If sidebar is dragged to be very small, change it to "none" state
                if (leftWidth < SIDEBAR_COLLAPSE_THRESHOLD) {
                  changeAppLayoutState({
                    activeSidebar: SIDEBAR_POSITION_NONE,
                    sidebarSetting: SIDEBAR_POSITION_LEFT,
                  });
                }
              } else if (isRightSidebar) {
                const rightWidth = sizes[sizes.length - 1];
                sideBarSetWidth(rightWidth);

                // If sidebar is dragged to be very small, change it to "none" state
                if (rightWidth < SIDEBAR_COLLAPSE_THRESHOLD) {
                  changeAppLayoutState({
                    activeSidebar: SIDEBAR_POSITION_NONE,
                    sidebarSetting: SIDEBAR_POSITION_RIGHT,
                  });
                }
              }
            }
          }}
        >
          {/* Left Sidebar */}
          {shouldShowSidebar && isLeftSidebar ? (
            <ResizablePanel
              priority={LayoutPriority["Normal"]}
              minSize={MIN_SIDEBAR_WIDTH}
              maxSize={MAX_SIDEBAR_WIDTH}
              preferredSize={sideBarGetWidth() || DEFAULT_SIDEBAR_WIDTH}
              className={`select-none ${isResizing ? "" : "transition-all duration-200"}`}
            >
              <Sidebar isResizing={isResizing} />
            </ResizablePanel>
          ) : null}

          {/* Main Content + Bottom Pane */}
          <ResizablePanel priority={LayoutPriority["High"]}>
            <Resizable
              vertical
              onDragStart={() => setIsResizing(true)}
              onDragEnd={(sizes) => {
                setIsResizing(false);
                const [_, bottomPaneHeight] = sizes;
                bottomPaneSetHeight(bottomPaneHeight);

                // If bottom pane is dragged to be very small, hide it
                if (bottomPaneHeight < BOTTOM_PANE_COLLAPSE_THRESHOLD) {
                  bottomPaneSetVisibility(false);
                }
              }}
            >
              <ResizablePanel>
                <MainContent />
              </ResizablePanel>
              {bottomPaneVisibility && (
                <ResizablePanel preferredSize={bottomPaneGetHeight()} minSize={MIN_BOTTOM_PANE_DRAGGABLE_HEIGHT}>
                  <BottomPane />
                </ResizablePanel>
              )}
            </Resizable>
          </ResizablePanel>

          {/* Right Sidebar */}
          {shouldShowSidebar && isRightSidebar ? (
            <ResizablePanel
              priority={LayoutPriority["Normal"]}
              minSize={MIN_SIDEBAR_WIDTH}
              maxSize={MAX_SIDEBAR_WIDTH}
              preferredSize={sideBarGetWidth() || DEFAULT_SIDEBAR_WIDTH}
              className={`select-none ${isResizing ? "" : "transition-all duration-200"}`}
            >
              <Sidebar isResizing={isResizing} />
            </ResizablePanel>
          ) : null}
        </Resizable>
      </div>

      {/* Bottom pane edge handler - visible only when bottom pane is hidden */}
      {!bottomPaneVisibility && <BottomPaneEdgeHandler onClick={handleShowBottomPane} />}
    </div>
  );
};

// Common sidebar content that can appear in either the left or right sidebar
const MainContent = () => (
  <ContentLayout className="relative flex h-full flex-col overflow-auto">
    <Suspense fallback={<div>Loading...</div>}>
      <TabbedPane theme="dockview-theme-light" />
    </Suspense>
  </ContentLayout>
);
