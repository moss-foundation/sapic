import React from "react";

import { useTabbedPaneStore } from "@/store/tabbedPane";

interface UseTabbedPaneResizeObserverProps {
  containerRef: React.RefObject<HTMLDivElement | null>;
}
export const useTabbedPaneResizeObserver = ({ containerRef }: UseTabbedPaneResizeObserverProps) => {
  const { api } = useTabbedPaneStore();

  React.useEffect(() => {
    if (!containerRef.current || !api) return;

    const resizeObserver = new ResizeObserver((entries) => {
      api.layout(entries[0].contentRect.width, entries[0].contentRect.height);
    });

    resizeObserver.observe(containerRef.current);

    return () => resizeObserver.disconnect();
  }, [api, containerRef]);
};
