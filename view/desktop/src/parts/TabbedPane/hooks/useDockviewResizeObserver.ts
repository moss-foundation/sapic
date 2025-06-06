import React from "react";

import { DockviewApi } from "@repo/moss-tabs";

export const useTabbedPaneResizeObserver = (
  api: DockviewApi | undefined,
  containerRef: React.RefObject<HTMLDivElement>
) => {
  React.useEffect(() => {
    if (!containerRef.current || !api) return;

    const resizeObserver = new ResizeObserver((entries) => {
      api.layout(entries[0].contentRect.width, entries[0].contentRect.height);
    });

    resizeObserver.observe(containerRef.current);

    return () => resizeObserver.disconnect();
  }, [api, containerRef]);
};
