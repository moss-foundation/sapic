import { LayoutPriority } from "allotment";
import { Suspense, useState } from "react";

import { useGetAppLayoutState } from "@/hooks/useGetAppLayoutState";
import { useGetActivityBarState } from "@/hooks/useGetActivityBarState";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import "@repo/moss-tabs/assets/styles.css";

import { Scrollbar, Sidebar } from "@/components";
import { logEntries } from "@/data/logEntries";

import { Resizable, ResizablePanel } from "../components/Resizable";
import TabbedPane from "../parts/TabbedPane/TabbedPane";
import { ContentLayout } from "./ContentLayout";

export const AppLayout = () => {
  const { data: appLayoutState } = useGetAppLayoutState();
  const { data: activityBarState } = useGetActivityBarState();

  const sideBarSetWidth = useAppResizableLayoutStore((state) => state.sideBar.setWidth);
  const sideBarGetWidth = useAppResizableLayoutStore((state) => state.sideBar.getWidth);

  const bottomPaneVisibility = useAppResizableLayoutStore((state) => state.bottomPane.visibility);
  const bottomPaneSetHeight = useAppResizableLayoutStore((state) => state.bottomPane.setHeight);
  const bottomPaneGetHeight = useAppResizableLayoutStore((state) => state.bottomPane.getHeight);

  // Determine if sidebar should be visible and which side it should be on
  const sidebarVisible = appLayoutState?.activeSidebar !== "none";
  const sidebarSide = appLayoutState?.sidebarSetting || "left";
  const isLeftSidebar = sidebarSide === "left";
  const isRightSidebar = sidebarSide === "right";

  // Check if we should display ActivityBar independently
  // (sidebar hidden + ActivityBar in default position)
  const shouldShowActivityBarAlone = !sidebarVisible && activityBarState?.position === "default";

  return (
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
        } else if (shouldShowActivityBarAlone) {
          // When ActivityBar is shown alone, ensure width is saved
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
      {/* Left Sidebar or standalone ActivityBar */}
      {(sidebarVisible && isLeftSidebar) || (shouldShowActivityBarAlone && isLeftSidebar) ? (
        <ResizablePanel
          priority={LayoutPriority["Normal"]}
          minSize={shouldShowActivityBarAlone && !sidebarVisible ? 41 : 150}
          maxSize={shouldShowActivityBarAlone && !sidebarVisible ? 41 : 400}
          preferredSize={shouldShowActivityBarAlone && !sidebarVisible ? 41 : sideBarGetWidth() || 270}
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
            <ResizablePanel preferredSize={bottomPaneGetHeight()} snap minSize={100}>
              <BottomPaneContent />
            </ResizablePanel>
          )}
        </Resizable>
      </ResizablePanel>

      {/* Right Sidebar or standalone ActivityBar */}
      {(sidebarVisible && isRightSidebar) || (shouldShowActivityBarAlone && isRightSidebar) ? (
        <ResizablePanel
          priority={LayoutPriority["Normal"]}
          minSize={shouldShowActivityBarAlone && !sidebarVisible ? 41 : 150}
          maxSize={shouldShowActivityBarAlone && !sidebarVisible ? 41 : 400}
          preferredSize={shouldShowActivityBarAlone && !sidebarVisible ? 41 : sideBarGetWidth() || 270}
          snap
          className="select-none"
        >
          <Sidebar />
        </ResizablePanel>
      ) : null}
    </Resizable>
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
        {logEntries.map((log, index) => (
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
