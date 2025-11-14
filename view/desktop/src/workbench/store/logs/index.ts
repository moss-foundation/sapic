import { create } from "zustand";

import { LogEntryInfo } from "@repo/window";

interface LogsStore {
  logs: LogEntryInfo[];
  errorCount: number;
  warnCount: number;
  appendLog: (log: LogEntryInfo) => void;
  clearLogs: () => void;
}

export const useLogsStore = create<LogsStore>((set) => ({
  logs: [],
  errorCount: 0,
  warnCount: 0,

  appendLog: (log) =>
    set((state) => {
      const errorDelta = log.level === "ERROR" ? 1 : 0;
      const warnDelta = log.level === "WARN" ? 1 : 0;

      return {
        logs: [...state.logs, log],
        errorCount: state.errorCount + errorDelta,
        warnCount: state.warnCount + warnDelta,
      };
    }),

  clearLogs: () =>
    set({
      logs: [],
      errorCount: 0,
      warnCount: 0,
    }),
}));
