import { useEffect, useRef, useState } from "react";

import { cn } from "@/utils";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

export const TestDropTarget = () => {
  const ref = useRef<HTMLDivElement | null>(null);
  const label = "Drop target for element";

  const [dropAllowance, setDropAllowance] = useState<boolean | null>(null);

  const [lastDroppedElement, setLastDroppedElement] = useState<{
    id: unknown;
    name: unknown;
    type: unknown;
    TreeId: unknown;
  } | null>(null);

  useEffect(() => {
    const element = ref?.current;

    if (!element) return;

    return dropTargetForElements({
      element,

      onDragEnter({ self, source, location }) {
        // console.log("onDragEnter", { self, source, location });
        setDropAllowance(true);
      },
      onDrag({ self, source, location }) {
        // console.log("onDrag", { self, source, location });
      },
      onDragLeave(args) {
        // console.log("onDragLeave", args);
        setDropAllowance(null);
      },
      onDrop({ self, source, location }) {
        // console.log("onDrop", { self, source, location });
        setDropAllowance(null);
        console.log(1234, { ...source.data });
        setLastDroppedElement({ ...source.data });
      },
    });
  }, [label, ref]);

  return (
    <div
      ref={ref}
      className={cn("absolute h-full w-[500px] inset-x-100  bg-amber-300", {
        "bg-amber-500": dropAllowance === null,
        "bg-green-600": dropAllowance === true,
      })}
    >
      {lastDroppedElement ? (
        <div className="relative h-full">
          <div className="overflow-auto h-full">
            <div>Last Dropped Item:</div>
            <pre>
              <code>{JSON.stringify(lastDroppedElement, null, 2)}</code>
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
