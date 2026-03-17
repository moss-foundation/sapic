import { useRef } from "react";

import { cn } from "@/utils";
import { IconInline } from "@/workbench/ui/components/IconInline";

import { useDropTargetProjectCreationZone } from "./dnd/useDropTargetProjectCreationZone";
import { useMonitorProjectCreationZone } from "./dnd/useMonitorProjectCreationZone";

export const ProjectCreationZone = () => {
  const ref = useRef<HTMLDivElement>(null);

  useMonitorProjectCreationZone();
  const { instruction } = useDropTargetProjectCreationZone({ ref });

  return (
    <div
      ref={ref}
      className={cn(
        "background-(--moss-accent-secondary) border-(--moss-accent) grid h-max min-h-32 w-full place-items-center rounded border-2 border-dashed transition-[translate] duration-100",
        {
          "background-(--moss-accent-secondary) -translate-y-1": instruction?.blocked === false,
        }
      )}
    >
      <div className="bg-size-[20px_20px] animate-stripes flex flex-col items-center justify-center gap-3 bg-[linear-gradient(-45deg,white_5%,transparent_5%_45%,white_45%_55%,transparent_55%_95%,white_95%)] p-8 text-center">
        <IconInline icon="AddCircleActive" className={cn("text-(--moss-accent) size-5 rounded-full")} />
        <span>Drag & drop selected items here to create a new project</span>
      </div>
    </div>
  );
};
