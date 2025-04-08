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
        onDragEnd={(sizes) => {
          if (primarySideBarPosition === "left") {
            const [leftPanelSize, _mainPanelSize] = sizes;
            primarySideBar.setWidth(leftPanelSize);
          }
          if (primarySideBarPosition === "right") {
            const [_mainPanelSize, rightPanelSize] = sizes;
            primarySideBar.setWidth(rightPanelSize);
          }
        }}
        onVisibleChange={(index, visible) => {
          if (primarySideBarPosition === "left" && index === 0) primarySideBar.setVisible(visible);
          if (primarySideBarPosition === "right" && index === 1) primarySideBar.setVisible(visible);
        }}
      >
        {primarySideBar.visible && primarySideBarPosition === "left" && (
          <ResizablePanel
            preferredSize={primarySideBar.width}
            visible={primarySideBar.visible && primarySideBarPosition === "left"}
            minSize={primarySideBar.minWidth}
            snap
            className="background-(--moss-primary-background)"
          >
            {SideBarPaneContent}
          </ResizablePanel>
        )}

        <ResizablePanel>
          <Resizable vertical>
            <ResizablePanel>{MainPaneContent}</ResizablePanel>
            <ResizablePanel visible={bottomPane.visible} minSize={bottomPane.minHeight} snap>
              {BottomPaneContent}
            </ResizablePanel>
          </Resizable>
        </ResizablePanel>

        {primarySideBar.visible && primarySideBarPosition === "right" && (
          <ResizablePanel
            preferredSize={primarySideBar.width}
            visible={primarySideBar.visible && primarySideBarPosition === "right"}
            minSize={primarySideBar.minWidth}
            snap
            className="background-(--moss-primary-background)"
          >
            {SideBarPaneContent}
          </ResizablePanel>
        )}
      </Resizable>

      {position === "default" && primarySideBarPosition === "right" && ActivityBar}
    </div>
  );
};

const SidebarContent = () => <Sidebar />;

const MainContent = () => (
  <ContentLayout className="background-(--moss-primary-background) relative flex h-full flex-col overflow-auto">
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
