import { create } from "zustand";

const colors = [
  "rgba(255,0,0,0.2)",
  "rgba(0,255,0,0.2)",
  "rgba(0,0,255,0.2)",
  "rgba(255,255,0,0.2)",
  "rgba(0,255,255,0.2)",
  "rgba(255,0,255,0.2)",
];
let count = 0;

interface DockviewLoggerStore {
  logLines: { text: string; timestamp?: Date; backgroundColor?: string }[];
  add: (logText: string) => void;
  clear: () => void;
}

export const useDockviewLoggerStore = create<DockviewLoggerStore>((set) => ({
  logLines: [],
  add: (logText: string) => {
    const color = colors[count++ % colors.length];

    const newLogLine = { text: logText, timestamp: new Date(), backgroundColor: color };

    set((state) => ({
      logLines: [...state.logLines, newLogLine],
    }));
  },
  clear: () => {
    set({ logLines: [] });
  },
}));
