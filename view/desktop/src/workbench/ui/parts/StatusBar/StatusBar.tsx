import { useEffect, useState, type ComponentPropsWithoutRef } from "react";

import { Icons } from "@/lib/ui";
import { cn } from "@/utils";
import { swapListById } from "@/utils/swapListById";
import { Divider } from "@/workbench/ui/components/Divider";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { StatusBarActivity } from "./StatusBarActivity";
import { StatusBarButton } from "./StatusBarButton";
import { StatusBarFPSCounter } from "./StatusBarFPSCounter";
import { StatusBarIndicators } from "./StatusBarIndicators";
import ZoomButtons from "./ZoomButtons";

interface Item {
  id: number;
  icon: Icons;
  label: string;
  order: number;
}

export const StatusBar = ({ className }: ComponentPropsWithoutRef<"div">) => {
  const [isOnline, setIsOnline] = useState(true);
  const [DNDList, setDNDList] = useState<Item[]>([
    {
      id: 1,
      icon: "Console",
      label: "Console",
      order: 1,
    },
    {
      id: 2,
      icon: "Trash",
      label: "Trash",
      order: 2,
    },
    {
      id: 3,
      icon: "Cookie",
      label: "Cookies",
      order: 3,
    },
  ]);

  useEffect(() => {
    return monitorForElements({
      onDrop({ location, source }) {
        const target = location.current.dropTargets[0];
        if (!target || target.data.draggableType !== "StatusBarButton") return;

        const sourceData = source.data;
        const targetData = target.data;
        if (!sourceData || !targetData) return;

        const updatedItems = swapListById(sourceData.id as number, targetData.id as number, DNDList);

        if (!updatedItems) return;

        setDNDList(updatedItems);
      },
    });
  }, [DNDList]);

  useEffect(() => {
    const handleOnline = () => setIsOnline(true);
    const handleOffline = () => setIsOnline(false);

    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);

    return () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    };
  }, []);

  return (
    <footer
      className={cn(
        "background-(--moss-statusBar-background) border-t-(--moss-border) flex w-screen justify-between border-t pl-1.5 pr-4",
        className
      )}
    >
      <div className="flex h-full gap-1">
        <div className="flex h-full gap-1">
          {DNDList.map((item) => (
            <StatusBarButton key={item.id} {...item} isDraggable draggableType="StatusBarButton" />
          ))}
        </div>

        <Divider className="py-1.5" />

        <StatusBarIndicators />
        <StatusBarActivity />
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
