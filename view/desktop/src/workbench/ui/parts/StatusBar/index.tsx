import { type ComponentPropsWithoutRef } from "react";

import { cn } from "@/utils";
import { Divider } from "@/workbench/ui/components/Divider";

import { StatusBarActivityPlaceholder } from "./components/StatusBarActivityPlaceholder";
import { StatusBarButton } from "./components/StatusBarButton";
import { StatusBarDraggableButton } from "./components/StatusBarDraggableButton";
import { StatusBarFPSCounter } from "./components/StatusBarFPSCounter";
import { StatusBarIndicatorsPlaceholder } from "./components/StatusBarIndicatorsPlaceholder";
import ZoomButtons from "./components/ZoomButtons";
import { useMonitorStatusBar } from "./dnd/hooks/useMonitorStatusBar";
import { useOnlineStatus } from "./hooks/useOnlineStatus";
import { useSyncStatusBarItems } from "./hooks/useSyncStatusBarItems";

export const StatusBar = ({ className }: ComponentPropsWithoutRef<"div">) => {
  const { isOnline } = useOnlineStatus();
  const { items } = useSyncStatusBarItems();

  useMonitorStatusBar();

  return (
    <footer
      className={cn(
        "background-(--moss-primary-background) border-t-(--moss-border) flex w-screen justify-between border-t pl-1.5 pr-4",
        className
      )}
    >
      <div className="flex h-full gap-1">
        <div className="flex h-full gap-1">
          {items.map((item) => (
            <StatusBarDraggableButton key={item.id} statusBarItem={item} />
          ))}
        </div>

        <Divider className="py-1.5" />

        <StatusBarIndicatorsPlaceholder />
        <StatusBarActivityPlaceholder />
      </div>

      <div className="flex h-full gap-6">
        <ZoomButtons />
        <div className="flex gap-1">
          <StatusBarFPSCounter />
          <Divider className="py-1.5" />
          <StatusBarButton icon={isOnline ? "Success" : "Error"} label={isOnline ? "Online" : "Offline"} />
        </div>
      </div>
    </footer>
  );
};
