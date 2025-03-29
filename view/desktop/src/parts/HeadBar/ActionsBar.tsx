import { HTMLProps } from "react";

import { ActionsGroup } from "@/components/ActionsGroup";
import { useChangeAppLayoutState, useGetAppLayoutState } from "@/hooks/useAppLayoutState";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";

export const ActionsBar = ({ className, ...props }: HTMLProps<HTMLDivElement>) => {
  const { data: appLayoutState } = useGetAppLayoutState();
  const { mutate: changeAppLayoutState } = useChangeAppLayoutState();

  const { bottomPane } = useAppResizableLayoutStore((state) => state);

  const toggleSidebar = (position: "left" | "right") => {
    if (!appLayoutState) return;

    if (appLayoutState.activeSidebar === position) {
      changeAppLayoutState({
        activeSidebar: "none",
      });
    } else {
      changeAppLayoutState({
        activeSidebar: position,
      });
    }
  };

  return (
    <div className={cn("flex items-center gap-3", className)} {...props}>
      <div className="flex items-center">
        {/* Left sidebar toggle button */}
        <ActionsGroup
          icon={appLayoutState?.activeSidebar === "left" ? "HeadBarLeftSideBarActive" : "HeadBarLeftSideBar"}
          iconClassName="size-[18px]"
          className="size-[30px]"
          onClick={() => toggleSidebar("left")}
        />

        {/* Bottom panel toggle button */}
        <ActionsGroup
          icon={bottomPane.visibility ? "HeadBarPanelActive" : "HeadBarPanel"}
          iconClassName="size-[18px]"
          className="size-[30px]"
          onClick={() => bottomPane.setVisibility(!bottomPane.visibility)}
        />

        {/* Right sidebar toggle button */}
        <ActionsGroup
          icon={appLayoutState?.activeSidebar === "right" ? "HeadBarRightSideBarActive" : "HeadBarRightSideBar"}
          iconClassName="size-[18px]"
          className="size-[30px]"
          onClick={() => toggleSidebar("right")}
        />
      </div>
    </div>
  );
};
