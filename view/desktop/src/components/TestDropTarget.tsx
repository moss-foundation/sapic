import { useEffect, useRef, useState } from "react";

import { cn } from "@/utils";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

export const TestDropTarget = () => {
  const ref = useRef<HTMLDivElement | null>(null);
  const label = "Drop target for element";

  const [dropAllowance, setDropAllowance] = useState<boolean | null>(null);

  const [lastDroppedElement, setLastDroppedElement] = useState<unknown | null>(null);

  useEffect(() => {
    const element = ref?.current;

    if (!element) return;

    return dropTargetForElements({
      element,

      onDragEnter() {
        setDropAllowance(true);
      },

      onDragLeave() {
        setDropAllowance(null);
      },
      onDrop({ source }) {
        setLastDroppedElement({ ...source.data });
        setDropAllowance(null);
      },
    });
  }, [label, ref]);

  return (
    <div
      ref={ref}
      className={cn("absolute h-full w-[500px] inset-x-100", {
        "bg-white": dropAllowance === null,
        "bg-green-300": dropAllowance === true,
      })}
    >
      {lastDroppedElement ? (
        <div className="relative h-full">
          <div className="overflow-auto h-full text-sm">
            <div>Last Dropped Item:</div>
            <pre>
              <code className="text-xs">{JSON.stringify(lastDroppedElement, null, 2)}</code>
            </pre>
          </div>
        </div>
      ) : (
        label
      )}
    </div>
  );
};

export default TestDropTarget;
