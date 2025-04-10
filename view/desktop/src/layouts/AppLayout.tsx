import { Suspense, useState } from "react";

import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import "@repo/moss-tabs/assets/styles.css";

import { testLogEntries } from "@/assets/testLogEntries";
import { ActivityBar, Scrollbar, Sidebar } from "@/components";
import { useActivityBarStore } from "@/store/activityBar";
import { cn } from "@/utils";

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

  const handleSidebarEdgeHandlerClick = () => {
    if (!primarySideBar.visible) primarySideBar.setVisible(true);
  };

  return (
    <div className="flex h-full w-full">
      {position === "default" && primarySideBarPosition === "left" && ActivityBar}
      <div className="relative flex h-full w-full">
        {!primarySideBar.visible && primarySideBarPosition === "left" && (
          <SidebarEdgeHandler alignment="left" onClick={handleSidebarEdgeHandlerClick} />
        )}

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
          {primarySideBarPosition === "left" && (
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

          {primarySideBarPosition === "right" && (
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

        {!primarySideBar.visible && primarySideBarPosition === "right" && (
          <SidebarEdgeHandler alignment="right" onClick={handleSidebarEdgeHandlerClick} />
        )}
      </div>

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
              <span className="mr-2 text-(--moss-secondary-text)">{log.timestamp}</span>
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

interface SidebarEdgeHandlerProps {
  alignment?: "left" | "right";
  onClick?: () => void;
}

const SidebarEdgeHandler = ({ alignment, onClick }: SidebarEdgeHandlerProps) => {
  return (
    <div
      className={cn("group/openHandle absolute z-40 h-full w-2 cursor-pointer", {
        "left-0": alignment === "left",
        "right-0": alignment === "right",
      })}
      onClick={onClick}
    >
      <div
        className={cn(
          "background-(--moss-info-background-hover)/70 absolute top-0 z-40 h-full w-3 opacity-0 transition-[opacity] duration-100 group-hover/openHandle:opacity-100",
          {
            "left-0": alignment === "left",
            "right-0": alignment === "right",
          }
        )}
      />

      <div
        className={cn(
          "background-(--moss-info-icon)/50 group-hover/openHandle:background-(--moss-info-icon)/80 absolute inset-y-[calc(50%-64px)] z-40 h-32 w-1.5 rounded transition-[opacity,translate] duration-100",
          {
            "left-[3px]": alignment === "left",
            "right-[3px]": alignment === "right",
          }
        )}
      />

      <div
        className={cn(
          "background-(--moss-info-icon) absolute inset-y-[calc(50%-12px)] z-40 flex size-6 items-center justify-center rounded-full opacity-0 transition-[opacity,translate] duration-100 group-hover/openHandle:opacity-100",
          {
            "left-1 group-hover/openHandle:translate-x-0.5": alignment === "left",
            "right-1 group-hover/openHandle:-translate-x-0.5": alignment === "right",
          }
        )}
      >
        <svg
          width="16"
          height="16"
          viewBox="0 0 16 16"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          className={cn({
            "rotate-180": alignment === "right",
          })}
        >
          <path d="M6 3L11 8L6 13" stroke="white" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
        </svg>
      </div>
    </div>
  );
};
