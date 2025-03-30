import { LayoutPriority } from "allotment";
import { Suspense, useState } from "react";

import { useGetAppLayoutState } from "@/hooks/useGetAppLayoutState";
import { useGetActivityBarState } from "@/hooks/useGetActivityBarState";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import "@repo/moss-tabs/assets/styles.css";

import { Scrollbar, Sidebar } from "@/components";
import { VerticalActivityBar } from "@/parts/ActivityBar/VerticalActivityBar";
import { testLogEntries } from "@/assets/testLogEntries";

import { Resizable, ResizablePanel } from "../components/Resizable";
import TabbedPane from "../parts/TabbedPane/TabbedPane";
import { ContentLayout } from "./ContentLayout";

const ACTIVITY_BAR_WIDTH = 41;
const DEFAULT_SIDEBAR_WIDTH = 270;
const MIN_SIDEBAR_WIDTH = 41;
const MAX_SIDEBAR_WIDTH = 400;
const MIN_BOTTOM_PANE_HEIGHT = 100;

const SIDEBAR_POSITION_LEFT = "left";
const SIDEBAR_POSITION_RIGHT = "right";
const SIDEBAR_POSITION_NONE = "none";

export const AppLayout = () => {
  const { data: appLayoutState } = useGetAppLayoutState();
  const { data: activityBarState } = useGetActivityBarState();

  const sideBarSetWidth = useAppResizableLayoutStore((state) => state.sideBar.setWidth);
  const sideBarGetWidth = useAppResizableLayoutStore((state) => state.sideBar.getWidth);

  const bottomPaneVisibility = useAppResizableLayoutStore((state) => state.bottomPane.visibility);
  const bottomPaneSetHeight = useAppResizableLayoutStore((state) => state.bottomPane.setHeight);
  const bottomPaneGetHeight = useAppResizableLayoutStore((state) => state.bottomPane.getHeight);

  const sidebarVisible = appLayoutState?.activeSidebar !== SIDEBAR_POSITION_NONE;
  const sidebarSide = appLayoutState?.sidebarSetting || SIDEBAR_POSITION_LEFT;
  const isLeftSidebar = sidebarSide === SIDEBAR_POSITION_LEFT;
  const isRightSidebar = sidebarSide === SIDEBAR_POSITION_RIGHT;

  const isActivityBarDefault = activityBarState?.position === "default";

  const shouldRenderStandaloneActivityBar = isActivityBarDefault;

  const shouldShowSidebar = sidebarVisible;

  return (
    <div className="relative h-full w-full">
      {/* Standalone VerticalActivityBar when in default position */}
      {shouldRenderStandaloneActivityBar && <VerticalActivityBar position={sidebarSide} />}

      <div
        className="h-full w-full"
        style={{
          paddingLeft: shouldRenderStandaloneActivityBar && isLeftSidebar ? ACTIVITY_BAR_WIDTH : 0,
          paddingRight: shouldRenderStandaloneActivityBar && isRightSidebar ? ACTIVITY_BAR_WIDTH : 0,
        }}
      >
        <Resizable
          onDragEnd={(sizes) => {
            if (sidebarVisible) {
              if (isLeftSidebar) {
                const [leftWidth] = sizes;
                sideBarSetWidth(leftWidth);
              } else if (isRightSidebar) {
                const [_, __, rightWidth] = sizes;
                sideBarSetWidth(rightWidth);
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
              snap
              className="select-none"
            >
              <Sidebar />
            </ResizablePanel>
          ) : null}

          {/* Main Content + Bottom Pane */}
          <ResizablePanel priority={LayoutPriority["High"]}>
            <Resizable
              vertical
              onDragEnd={(sizes) => {
                const [_, bottomPaneHeight] = sizes;
                bottomPaneSetHeight(bottomPaneHeight);
              }}
            >
              <ResizablePanel>
                <MainContent />
              </ResizablePanel>
              {bottomPaneVisibility && (
                <ResizablePanel preferredSize={bottomPaneGetHeight()} snap minSize={MIN_BOTTOM_PANE_HEIGHT}>
                  <BottomPaneContent />
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
              snap
              className="select-none"
            >
              <Sidebar />
            </ResizablePanel>
          ) : null}
        </Resizable>
      </div>
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

const BottomPaneContent = () => {
  const [isHovering, setIsHovering] = useState(false);

  return (
    <Scrollbar
      className="h-full overflow-auto"
      onMouseEnter={() => setIsHovering(true)}
      onMouseLeave={() => setIsHovering(false)}
    >
      <div className={`p-2 font-mono text-sm ${isHovering ? "select-text" : "select-none"}`}>
        <div className="mb-2 font-semibold">Application Logs:</div>
        {testLogEntries.map((log, index) => (
          <div key={index} className="mb-1 flex">
            <span className="mr-2 text-[var(--moss-text-secondary)]">{log.timestamp}</span>
            <span
              className={`mr-2 min-w-16 font-medium ${
                log.level === "ERROR"
                  ? "text-red-500"
                  : log.level === "WARNING"
                    ? "text-amber-500"
                    : log.level === "DEBUG"
                      ? "text-blue-500"
                      : "text-green-500"
              }`}
            >
              {log.level}
            </span>
            <span className="mr-2 min-w-32 font-semibold">{log.service}:</span>
            <span>{log.message}</span>
          </div>
        ))}
      </div>
    </Scrollbar>
  );
};
