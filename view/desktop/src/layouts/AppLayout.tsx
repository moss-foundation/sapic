import { Suspense, useState } from "react";

import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import "@repo/moss-tabs/assets/styles.css";

import { testLogEntries } from "@/assets/testLogEntries";
import { ActivityBar, Scrollbar, Sidebar } from "@/components";
import { useActivityBarStore } from "@/store/activityBar";

import { Resizable, ResizablePanel } from "../components/Resizable";
import TabbedPane from "../parts/TabbedPane/TabbedPane";
import { ContentLayout } from "./ContentLayout";

export const AppLayout = () => {
  return (
    <DefaultLayout
      SideBarPaneContent={<SidebarContent />}
      MainPaneContent={<MainContent />}
      BottomPaneContent={<BottomPaneContent />}
      ActivityBar={<ActivityBar />}
    />
  );
};

interface DefaultLayoutProps {
  SideBarPaneContent: JSX.Element;
  MainPaneContent: JSX.Element;
  BottomPaneContent: JSX.Element;
  ActivityBar: JSX.Element;
}

const DefaultLayout = ({ SideBarPaneContent, MainPaneContent, BottomPaneContent, ActivityBar }: DefaultLayoutProps) => {
  const { position } = useActivityBarStore();
  const { bottomPane, primarySideBar, primarySideBarPosition } = useAppResizableLayoutStore();

  return (
    <div className="flex h-full w-full">
      {position === "default" && primarySideBarPosition === "left" && ActivityBar}
      <Resizable
        smoothHide
        onDragEnd={(sizes) => {
          const [leftPanelSize, _mainPanelSize, rightPanelSize] = sizes;
          if (primarySideBarPosition === "left") primarySideBar.setWidth(leftPanelSize);
          if (primarySideBarPosition === "right") primarySideBar.setWidth(rightPanelSize);
        }}
        onVisibleChange={(index, visible) => {
          if (primarySideBarPosition === "left" && index === 0) primarySideBar.setVisible(visible);
          if (primarySideBarPosition === "right" && index === 2) primarySideBar.setVisible(visible);
        }}
      >
        <ResizablePanel
          preferredSize={primarySideBar.width}
          visible={primarySideBar.visible && primarySideBarPosition === "left"}
          minSize={primarySideBar.minWidth}
          snap
        >
          {primarySideBarPosition === "left" && SideBarPaneContent}
        </ResizablePanel>

        <ResizablePanel>
          <Resizable vertical>
            <ResizablePanel>{MainPaneContent}</ResizablePanel>
            <ResizablePanel visible={bottomPane.visible} minSize={bottomPane.minHeight} snap>
              {BottomPaneContent}
            </ResizablePanel>
          </Resizable>
        </ResizablePanel>

        <ResizablePanel
          preferredSize={primarySideBar.width}
          visible={primarySideBar.visible && primarySideBarPosition === "right"}
          minSize={primarySideBar.minWidth}
          snap
        >
          {primarySideBarPosition === "right" && SideBarPaneContent}
        </ResizablePanel>
      </Resizable>

      {position === "default" && primarySideBarPosition === "right" && ActivityBar}
    </div>
  );
};

const SidebarContent = () => <Sidebar />;

const MainContent = () => (
  <ContentLayout className="relative flex h-full flex-col overflow-auto bg-red-500">
    <Suspense fallback={<div>Loading...</div>}>
      <TabbedPane theme="dockview-theme-light" />
    </Suspense>
  </ContentLayout>
);

const BottomPaneContent = () => {
  const [isHovering, setIsHovering] = useState(false);

  return (
    <div className="background-(--moss-primary-background) h-full w-full">
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
    </div>
  );
};
