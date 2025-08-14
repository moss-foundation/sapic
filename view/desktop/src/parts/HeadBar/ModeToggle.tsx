import React, { useCallback, useEffect, useRef, useState } from "react";

import { Item as ToggleGroupItem, Root as ToggleGroupRoot } from "@/components/ToggleGroup";
import { useStreamedCollectionsWithEntries } from "@/hooks";
import { useRequestModeStore } from "@/store/requestMode";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";

interface ModeToggleProps {
  className?: string;
  compact?: boolean;
}

export const ModeToggle: React.FC<ModeToggleProps> = ({ className, compact = false }) => {
  const itemsRef = useRef<{ [key: string]: HTMLButtonElement | null }>({});
  const containerRef = useRef<HTMLDivElement>(null);

  const { api } = useTabbedPaneStore();
  const { displayMode, setDisplayMode } = useRequestModeStore();
  const { data: collectionsWithEntries } = useStreamedCollectionsWithEntries();

  const [sliderStyle, setSliderStyle] = useState({ width: 0, left: 0 });

  const updateSliderPosition = useCallback(() => {
    const activeItem = itemsRef.current[displayMode];
    if (activeItem) {
      const { width, left } = activeItem.getBoundingClientRect();
      const parentLeft = activeItem.parentElement?.getBoundingClientRect().left || 0;
      setSliderStyle({
        width,
        left: left - parentLeft,
      });
    }
  }, [displayMode]);

  useEffect(() => {
    updateSliderPosition();

    const resizeObserver = new ResizeObserver(() => {
      updateSliderPosition();
    });

    if (containerRef.current) {
      resizeObserver.observe(containerRef.current);
    }

    return () => {
      resizeObserver.disconnect();
    };
  }, [displayMode, updateSliderPosition]);

  const handleSetRequestFirstMode = () => {
    setDisplayMode("REQUEST_FIRST");
    const allEntries = collectionsWithEntries?.map((collection) => collection.entries).flat();

    allEntries?.forEach((entry) => {
      if (entry.class !== "Request" || entry.path.segments.length === 1) {
        const panel = api?.getPanel(entry.id);
        if (panel) {
          api?.removePanel(panel);
        }
      }
    });
  };

  const handleSetDesignFirstMode = () => {
    setDisplayMode("DESIGN_FIRST");
  };
  return (
    <ToggleGroupRoot
      type="single"
      value={displayMode}
      className={cn("relative rounded-sm border border-[var(--moss-border-color)]", className)}
    >
      <div className="relative flex" ref={containerRef}>
        <div
          className="absolute h-[24px] rounded-sm bg-white transition-all duration-300 ease-in-out"
          style={{
            width: `${sliderStyle.width}px`,
            left: `${sliderStyle.left}px`,
          }}
        />
        <ToggleGroupItem
          value="REQUEST_FIRST"
          className="relative z-10 whitespace-nowrap transition-colors duration-300"
          compact={compact}
          ref={(el) => (itemsRef.current["REQUEST_FIRST"] = el)}
          onClick={handleSetRequestFirstMode}
        >
          Request mode
        </ToggleGroupItem>
        <ToggleGroupItem
          value="DESIGN_FIRST"
          className="relative z-10 whitespace-nowrap transition-colors duration-300"
          compact={compact}
          ref={(el) => (itemsRef.current["DESIGN_FIRST"] = el)}
          onClick={handleSetDesignFirstMode}
        >
          Design mode
        </ToggleGroupItem>
      </div>
    </ToggleGroupRoot>
  );
};

export default ModeToggle;
