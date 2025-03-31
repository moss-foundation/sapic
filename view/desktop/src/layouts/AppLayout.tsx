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

import { LeftSidebar, RightSidebar } from "@/components";
import { VerticalActivityBar } from "@/parts/ActivityBar/VerticalActivityBar";
import { SidebarEdgeHandler } from "@/parts/SideBar/SidebarEdgeHandler";
import { BottomPane } from "@/parts/BottomPane/BottomPane";
import { BottomPaneEdgeHandler } from "@/parts/BottomPane/BottomPaneEdgeHandler";
import { Resizable, ResizablePanel } from "../components/Resizable";
import TabbedPane from "../parts/TabbedPane/TabbedPane";
import { ContentLayout } from "./ContentLayout";
import { cn } from "@/utils";

export const AppLayout = () => {
  const { data: appLayoutState } = useGetAppLayoutState();
  const { mutate: changeAppLayoutState } = useChangeAppLayoutState();
  const { data: activityBarState } = useGetActivityBarState();

  const sideBarSetWidth = useAppResizableLayoutStore((state) => state.sideBar.setWidth);
  const sideBarGetWidth = useAppResizableLayoutStore((state) => state.sideBar.getWidth);
  const lastSidebarWidthRef = useRef(sideBarGetWidth() || DEFAULT_SIDEBAR_WIDTH);

  const [isResizing, setIsResizing] = useState(false);
  const [leftSidebarWidth, setLeftSidebarWidth] = useState(DEFAULT_SIDEBAR_WIDTH);
  const [rightSidebarWidth, setRightSidebarWidth] = useState(DEFAULT_SIDEBAR_WIDTH);

  const bottomPaneVisibility = useAppResizableLayoutStore((state) => state.bottomPane.visibility);
  const bottomPaneSetHeight = useAppResizableLayoutStore((state) => state.bottomPane.setHeight);
  const bottomPaneGetHeight = useAppResizableLayoutStore((state) => state.bottomPane.getHeight);
  const bottomPaneSetVisibility = useAppResizableLayoutStore((state) => state.bottomPane.setVisibility);
  const lastBottomPaneHeightRef = useRef(bottomPaneGetHeight() || MIN_BOTTOM_PANE_HEIGHT);

  const sidebarVisible = appLayoutState?.activeSidebar !== SIDEBAR_POSITION_NONE;
  const sidebarSide = appLayoutState?.sidebarSetting || SIDEBAR_POSITION_LEFT;
  const isLeftSidebar = sidebarSide === SIDEBAR_POSITION_LEFT;
  const isRightSidebar = sidebarSide === SIDEBAR_POSITION_RIGHT;
  const isLeftSidebarActive = sidebarVisible && isLeftSidebar;
  const isRightSidebarActive = sidebarVisible && isRightSidebar;

  const isActivityBarDefault = activityBarState?.position === "default";
  const shouldRenderStandaloneActivityBar = isActivityBarDefault;

  // Track sidebar resizing state with refs
  const leftSidebarRef = useRef<HTMLDivElement>(null);
  const rightSidebarRef = useRef<HTMLDivElement>(null);
  const mainContentRef = useRef<HTMLDivElement>(null);
  const appLayoutRef = useRef<HTMLDivElement>(null);
  const startResizeXRef = useRef<number>(0);
  const isLeftSidebarResizingRef = useRef<boolean>(false);
  const isRightSidebarResizingRef = useRef<boolean>(false);

  // Update state when sidebar width changes
  useEffect(() => {
    const width = sideBarGetWidth() || DEFAULT_SIDEBAR_WIDTH;
    if (isLeftSidebarActive) {
      setLeftSidebarWidth(width);
    } else if (isRightSidebarActive) {
      setRightSidebarWidth(width);
    }
  }, [sideBarGetWidth, isLeftSidebarActive, isRightSidebarActive]);

  useEffect(() => {
    if (sidebarVisible && sideBarGetWidth() > MIN_SIDEBAR_WIDTH) {
      lastSidebarWidthRef.current = sideBarGetWidth();
    }
  }, [sidebarVisible, sideBarGetWidth]);

  useEffect(() => {
    if (bottomPaneVisibility && bottomPaneGetHeight() >= MIN_BOTTOM_PANE_HEIGHT) {
      lastBottomPaneHeightRef.current = bottomPaneGetHeight();
    }
  }, [bottomPaneVisibility, bottomPaneGetHeight]);

  // Handle mouse events for resizing sidebars
  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isResizing) return;

      if (isLeftSidebarResizingRef.current && leftSidebarRef.current) {
        const newWidth = Math.max(Math.min(e.clientX, MAX_SIDEBAR_WIDTH), MIN_SIDEBAR_WIDTH);
        setLeftSidebarWidth(newWidth);
      } else if (isRightSidebarResizingRef.current && rightSidebarRef.current && mainContentRef.current) {
        const mainContentRect = mainContentRef.current.getBoundingClientRect();
        const newWidth = Math.max(Math.min(mainContentRect.right - e.clientX, MAX_SIDEBAR_WIDTH), MIN_SIDEBAR_WIDTH);
        setRightSidebarWidth(newWidth);
      }
    };

    const handleMouseUp = () => {
      if (!isResizing) return;

      setIsResizing(false);
      isLeftSidebarResizingRef.current = false;
      isRightSidebarResizingRef.current = false;

      if (isLeftSidebarActive) {
        sideBarSetWidth(leftSidebarWidth);

        if (leftSidebarWidth < SIDEBAR_COLLAPSE_THRESHOLD) {
          changeAppLayoutState({
            activeSidebar: SIDEBAR_POSITION_NONE,
            sidebarSetting: SIDEBAR_POSITION_LEFT,
          });
        }
      } else if (isRightSidebarActive) {
        sideBarSetWidth(rightSidebarWidth);

        if (rightSidebarWidth < SIDEBAR_COLLAPSE_THRESHOLD) {
          changeAppLayoutState({
            activeSidebar: SIDEBAR_POSITION_NONE,
            sidebarSetting: SIDEBAR_POSITION_RIGHT,
          });
        }
      }
    };

    if (isResizing) {
      document.addEventListener("mousemove", handleMouseMove);
      document.addEventListener("mouseup", handleMouseUp);
    }

    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };
  }, [
    isResizing,
    leftSidebarWidth,
    rightSidebarWidth,
    isLeftSidebarActive,
    isRightSidebarActive,
    sideBarSetWidth,
    changeAppLayoutState,
  ]);

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

  // Start resizing for left sidebar
  const startLeftSidebarResize = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);
    isLeftSidebarResizingRef.current = true;
    startResizeXRef.current = e.clientX;
  };

  // Start resizing for right sidebar
  const startRightSidebarResize = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);
    isRightSidebarResizingRef.current = true;
    startResizeXRef.current = e.clientX;
  };

  return (
    <div ref={appLayoutRef} className="relative h-full w-full overflow-hidden">
      {/* Standalone VerticalActivityBar */}
      {shouldRenderStandaloneActivityBar && (
        <div
          className={cn("absolute top-0 bottom-0 z-20", isLeftSidebar ? "left-0" : "right-0")}
          style={{
            width: ACTIVITY_BAR_WIDTH,
            top: 0,
          }}
        >
          <VerticalActivityBar position={sidebarSide} />
        </div>
      )}

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

        {/* Main content area - always present */}
        <div
          ref={mainContentRef}
          className="h-full w-full"
          style={{
            paddingLeft: isLeftSidebarActive ? leftSidebarWidth : 0,
            paddingRight: isRightSidebarActive ? rightSidebarWidth : 0,
            transition: isResizing ? "none" : "padding 0.2s ease-in-out",
          }}
        >
          <Resizable
            vertical
            onDragStart={() => setIsResizing(true)}
            onDragEnd={(sizes) => {
              setIsResizing(false);
              const [_, bottomPaneHeight] = sizes;
              bottomPaneSetHeight(bottomPaneHeight);

              if (bottomPaneHeight < BOTTOM_PANE_COLLAPSE_THRESHOLD) {
                bottomPaneSetVisibility(false);
              }
            }}
          >
            <ResizablePanel>
              <ContentLayout className="relative flex h-full flex-col overflow-auto">
                <Suspense fallback={<div>Loading...</div>}>
                  <TabbedPane theme="dockview-theme-light" />
                </Suspense>
              </ContentLayout>
            </ResizablePanel>

            {bottomPaneVisibility && (
              <ResizablePanel preferredSize={bottomPaneGetHeight()} minSize={MIN_BOTTOM_PANE_DRAGGABLE_HEIGHT}>
                <BottomPane />
              </ResizablePanel>
            )}
          </Resizable>
        </div>

        {/* Left Sidebar */}
        <div
          ref={leftSidebarRef}
          className={cn(
            "absolute top-0 bottom-0 left-0 z-10",
            isResizing ? "" : "transition-transform duration-200 ease-in-out",
            isLeftSidebarActive ? "" : "-translate-x-full transform"
          )}
          style={{
            width: leftSidebarWidth,
            marginLeft: shouldRenderStandaloneActivityBar && isLeftSidebar ? ACTIVITY_BAR_WIDTH : 0,
            top: 0,
          }}
        >
          <LeftSidebar isResizing={isResizing} />

          {/* Resize handle */}
          <div
            className="absolute top-0 right-0 h-full w-2 cursor-ew-resize hover:bg-blue-500 hover:opacity-20"
            onMouseDown={startLeftSidebarResize}
          />
        </div>

        {/* Right Sidebar */}
        <div
          ref={rightSidebarRef}
          className={cn(
            "absolute top-0 right-0 bottom-0 z-10",
            isResizing ? "" : "transition-transform duration-200 ease-in-out",
            isRightSidebarActive ? "" : "translate-x-full transform"
          )}
          style={{
            width: rightSidebarWidth,
            marginRight: shouldRenderStandaloneActivityBar && isRightSidebar ? ACTIVITY_BAR_WIDTH : 0,
            top: 0,
          }}
        >
          <RightSidebar isResizing={isResizing} />

          {/* Resize handle */}
          <div
            className="absolute top-0 left-0 h-full w-2 cursor-ew-resize hover:bg-blue-500 hover:opacity-20"
            onMouseDown={startRightSidebarResize}
          />
        </div>
      </div>

      {/* Bottom pane edge handler - visible only when bottom pane is hidden */}
      {!bottomPaneVisibility && <BottomPaneEdgeHandler onClick={handleShowBottomPane} />}
    </div>
  );
};
