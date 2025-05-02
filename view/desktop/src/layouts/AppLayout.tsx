import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import "@repo/moss-tabs/assets/styles.css";

import { AllotmentHandle, LayoutPriority } from "allotment";
import { useEffect, useRef, useState } from "react";

import { ActivityBar, BottomPane, Sidebar } from "@/components";
import { useUpdatePanelPartState } from "@/hooks/appState/useUpdatePanelPartState";
import { useUpdateSidebarPartState } from "@/hooks/appState/useUpdateSidebarPartState";
import { useActivityBarStore } from "@/store/activityBar";
import { cn } from "@/utils";

import { Resizable, ResizablePanel } from "../components/Resizable";
import TabbedPane from "../parts/TabbedPane/TabbedPane";

export const AppLayout = () => {
  const canUpdatePartState = useRef(false);
  const numberOfRerenders = useRef(0);

  const { position } = useActivityBarStore();
  const { bottomPane, sideBar, sideBarPosition } = useAppResizableLayoutStore();

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

  const { mutate: updateSidebarPartState } = useUpdateSidebarPartState();
  useEffect(() => {
    if (!canUpdatePartState.current) return;

    updateSidebarPartState({
      preferredSize: sideBar.width,
      isVisible: sideBar.visible,
    });
  }, [sideBar, updateSidebarPartState]);

  const { mutate: updatePanelPartState } = useUpdatePanelPartState();
  useEffect(() => {
    if (!canUpdatePartState.current) return;

    updatePanelPartState({
      preferredSize: bottomPane.height,
      isVisible: bottomPane.visible,
    });
  }, [bottomPane, updatePanelPartState]);

  //FIXME this is a hack to prevent the part state from being updated on initial mount in strict mode.
  useEffect(() => {
    numberOfRerenders.current++;

    if (numberOfRerenders.current >= 2) {
      canUpdatePartState.current = true;
    }
  }, []);

  return (
    <div className="flex h-full w-full">
      {position === "default" && sideBarPosition === "left" && <ActivityBar />}
      <div className="relative flex h-full w-full">
        {!sideBar.visible && sideBarPosition === "left" && (
          <SidebarEdgeHandler alignment="left" onClick={handleSidebarEdgeHandlerClick} />
        )}

        <Resizable
          proportionalLayout={false}
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
            {!bottomPane.visible && (
              <SidebarEdgeHandler alignment="bottom" onClick={handleBottomPaneEdgeHandlerClick} />
            )}
          </ResizablePanel>

          {sideBarPosition === "right" && (
            <ResizablePanel
              preferredSize={sideBar.width}
              visible={sideBar.visible && sideBarPosition === "right"}
              minSize={sideBar.minWidth}
              maxSize={sideBar.maxWidth}
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
