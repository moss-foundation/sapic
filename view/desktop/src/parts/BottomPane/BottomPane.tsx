import { useEffect, useState } from "react";
import { Scrollbar } from "@/lib/ui";
import { LogEntryInfo, ON_DID_APPEND_LOG_ENTRY_CHANNEL } from "@repo/moss-app";
import { listen } from "@tauri-apps/api/event";

export const BottomPane = () => {
  const [logs, setLogs] = useState<LogEntryInfo[]>([]);

  useEffect(() => {
    const unlisten = listen(ON_DID_APPEND_LOG_ENTRY_CHANNEL, (event) => {
      const log: LogEntryInfo = event.payload as LogEntryInfo;
      setLogs([...logs, log]);
    });
    return () => {
      unlisten.then((unlistenFn) => unlistenFn());
    };
  });

  return (
    <div className="background-(--moss-primary-background) h-full w-full">
      <Scrollbar className="h-full">
        <div className={`select-none p-2 font-mono text-sm hover:select-text`}>
          <div className="mb-2 font-semibold">Application and Session Logs:</div>
          {logs.map((log, index) => (
            <div key={index} className="mb-1 flex">
              <span className="text-(--moss-secondary-foreground) mr-2">{log.timestamp}</span>
              <span
                className={`mr-2 min-w-16 font-medium ${
                  log.level === "ERROR"
                    ? "text-red-500"
                    : log.level === "WARN"
                      ? "text-amber-500"
                      : log.level === "DEBUG"
                        ? "text-blue-500"
                        : "text-green-500"
                }`}
              >
                {log.level}
              </span>
              <span className="mr-2 min-w-32 font-semibold">{log.resource || "No Resource"}:</span>
              <span>{log.message}</span>
            </div>
          ))}
        </div>
      </Scrollbar>
    </div>
  );
};
