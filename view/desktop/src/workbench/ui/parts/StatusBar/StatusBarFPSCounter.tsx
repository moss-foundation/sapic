import { useEffect, useRef, useState } from "react";

import { StatusBarButton } from "./StatusBarButton";

export const StatusBarFPSCounter = ({ updateInterval = 100 }) => {
  const [fps, setFps] = useState(0);
  const frames = useRef(0);
  const lastTime = useRef<number>(0);

  useEffect(() => {
    let animationFrameId: number;

    const loop = (time: number) => {
      frames.current++;

      const delta = time - lastTime.current;
      if (delta >= updateInterval) {
        setFps(Math.round((frames.current * 1000) / delta));
        frames.current = 0;
        lastTime.current = time;
      }

      animationFrameId = requestAnimationFrame(loop);
    };

    animationFrameId = requestAnimationFrame(loop);

    return () => cancelAnimationFrame(animationFrameId);
  }, [updateInterval]);

  return <StatusBarButton className="min-w-16" label={`${fps} FPS`} />;
};
