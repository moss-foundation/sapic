import { useEffect } from "react";

import { Scrollbar } from "@/lib/ui";
import { useLogsStore } from "@/store/logs";
import { cn } from "@/utils";
import { LogEntryInfo, ON_DID_APPEND_LOG_ENTRY_CHANNEL } from "@repo/window";
import { listen } from "@tauri-apps/api/event";

export const BottomPane = () => {
  const { logs, appendLog } = useLogsStore();

  useEffect(() => {
    const unlisten = listen(ON_DID_APPEND_LOG_ENTRY_CHANNEL, (event) => {
      const log: LogEntryInfo = event.payload as LogEntryInfo;
      appendLog(log);
    });

    return () => {
      unlisten.then((unlistenFn) => unlistenFn());
    };
  }, [appendLog]);

  return (
    <div className="background-(--moss-primary-background) h-full w-full">
      <Scrollbar className="h-full">
        <div className={`select-none p-2 font-mono text-sm hover:select-text`}>
          <div className="mb-2 font-semibold">Application and Session Logs:</div>
          {logs.map((log, index) => (
            <div key={index} className="mb-1 flex">
              <span className="text-(--moss-secondary-foreground) mr-2">{log.timestamp}</span>
              <span
                className={cn("mr-2 min-w-16 font-medium", {
                  "text-red-500": log.level === "ERROR",
                  "text-amber-500": log.level === "WARN",
                  "text-blue-500": log.level === "DEBUG",
                  "text-green-500": log.level === "INFO",
                })}
              >
                {log.level}
              </span>
              <span>{log.message}</span>
            </div>
          ))}
        </div>
      </Scrollbar>
    </div>
  );
};
