import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import "@repo/moss-tabs/assets/styles.css";

import { AllotmentHandle } from "allotment";
import { useEffect, useRef } from "react";

import { ActivityBar, BottomPane, Sidebar } from "@/components";
import { useActivityBarStore } from "@/store/activityBar";
import { cn } from "@/utils";

import { Resizable, ResizablePanel } from "../components/Resizable";
import TabbedPane from "../parts/TabbedPane/TabbedPane";

export const AppLayout = () => {
  const { position } = useActivityBarStore();
  const { bottomPane, sideBar, sideBarPosition } = useAppResizableLayoutStore();

  const handleSidebarEdgeHandlerClick = () => {
    if (!sideBar.visible) sideBar.setVisible(true);
  };

  const resizableRef = useRef<AllotmentHandle>(null);

  useEffect(() => {
    if (!resizableRef.current) return;

    resizableRef.current.reset();
  }, [bottomPane, sideBar, sideBarPosition]);

  return (
    <div className="flex h-full w-full">
      {position === "default" && sideBarPosition === "left" && <ActivityBar />}
      <div className="relative flex h-full w-full">
        {!sideBar.visible && sideBarPosition === "left" && (
          <SidebarEdgeHandler alignment="left" onClick={handleSidebarEdgeHandlerClick} />
        )}

        <Resizable
          ref={resizableRef}
          onDragEnd={(sizes) => {
            if (sideBarPosition === "left") {
              const [leftPanelSize, _mainPanelSize] = sizes;
              sideBar.setWidth(leftPanelSize);
            }
            if (sideBarPosition === "right") {
              const [_mainPanelSize, rightPanelSize] = sizes;
              sideBar.setWidth(rightPanelSize);
            }
          }}
          onVisibleChange={(index, visible) => {
            if (sideBarPosition === "left" && index === 0) sideBar.setVisible(visible);
            if (sideBarPosition === "right" && index === 1) sideBar.setVisible(visible);
          }}
        >
          {sideBarPosition === "left" && (
            <ResizablePanel
              preferredSize={sideBar.width}
              visible={sideBar.visible && sideBarPosition === "left"}
              minSize={sideBar.minWidth}
              maxSize={sideBar.maxWidth}
              snap
              className="background-(--moss-primary-background)"
            >
              <SidebarContent />
            </ResizablePanel>
          )}
          <ResizablePanel>
            <Resizable
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
              <ResizablePanel>
                <MainContent />
              </ResizablePanel>
              <ResizablePanel
                preferredSize={bottomPane.height}
                visible={bottomPane.visible}
                minSize={bottomPane.minHeight}
                snap
              >
                <BottomPaneContent />
              </ResizablePanel>
            </Resizable>
          </ResizablePanel>

          {sideBarPosition === "right" && (
            <ResizablePanel
              preferredSize={sideBar.width}
              visible={sideBar.visible && sideBarPosition === "right"}
              minSize={sideBar.minWidth}
              snap
              className="background-(--moss-primary-background)"
            >
              <SidebarContent />
            </ResizablePanel>
          )}
        </Resizable>

        {!sideBar.visible && sideBarPosition === "right" && (
          <SidebarEdgeHandler alignment="right" onClick={handleSidebarEdgeHandlerClick} />
        )}
      </div>

      {position === "default" && sideBarPosition === "right" && <ActivityBar />}
    </div>
  );
};

const SidebarContent = () => <Sidebar />;

const MainContent = () => <TabbedPane theme="dockview-theme-light" />;

const BottomPaneContent = () => {
  return <BottomPane />;
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
