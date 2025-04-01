import { useState } from "react";

import { testLogEntries } from "@/assets/testLogEntries";
import { Scrollbar } from "@/components";

export const BottomPane = () => {
  const [isHovering, setIsHovering] = useState(false);

  return (
    <div className="background-(--moss-primary-background) h-full w-full">
      <Scrollbar
        className="h-full overflow-auto"
        onMouseEnter={() => setIsHovering(true)}
        onMouseLeave={() => setIsHovering(false)}
      >
        <div className={`p-2 font-mono text-sm ${isHovering ? "select-text" : "select-none"}`}>
          <div className="mb-2 font-semibold">Application Logs:</div>
          {testLogEntries.map((log, index) => (
            <div key={index} className="mb-1 flex">
              <span className="mr-2 text-[var(--moss-text-secondary)]">{log.timestamp}</span>
              <span
                className={`mr-2 min-w-16 font-medium ${
                  log.level === "ERROR"
                    ? "text-red-500"
                    : log.level === "WARNING"
                      ? "text-amber-500"
                      : log.level === "DEBUG"
                        ? "text-blue-500"
                        : "text-green-500"
                }`}
              >
                {log.level}
              </span>
              <span className="mr-2 min-w-32 font-semibold">{log.service}:</span>
              <span>{log.message}</span>
            </div>
          ))}
        </div>
      </Scrollbar>
    </div>
  );
};
