import { useEffect, useState } from "react";

import { cn } from "@/utils";

// TODO: fetch tips from backend when they are available
const tips = [
  "The statusbar color can be changed in the appearance settings",
  "You can change the order of widget actions. Try it!",
  "Lorem ipsum dolor sit amat. Met the statusbar color can be changed in the appearance settings",
];

interface PageLoaderProps {
  className?: string;
}

export const PageLoader = ({ className }: PageLoaderProps) => {
  const [tip, setTip] = useState(tips[0]);

  useEffect(() => {
    const interval = setInterval(() => {
      const randomTip = tips[Math.floor(Math.random() * tips.length)];
      setTip(randomTip);
    }, 5000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className={cn("relative flex h-full w-full flex-col items-center justify-between bg-white pt-4", className)}>
      <div className="fixed top-0 h-36 w-full" data-tauri-drag-region />

      <div className="flex h-full flex-col items-center justify-center gap-5">
        <div className="size-8 animate-spin rounded-full border-2 border-gray-200 border-t-gray-600"></div>
        <div className="flex flex-col gap-3 text-center">
          <div className="font-black">Did you know</div>
          <div className="text animate-text-slide text-[#6F6F6F]">{tip}</div>
        </div>
      </div>

      <div className="pb-4 text-xs text-[#525252]">This may take a few seconds.</div>
    </div>
  );
};

export default PageLoader;
