import React, { useEffect, useState } from "react";

import { listen } from "@tauri-apps/api/event";

export const Logs: React.FC = () => {
  const [logs, setLogs] = useState<string[]>([]);

  useEffect(() => {
    // Listen for logs from the backend
    const unlisten = listen<string>("logs-stream", (event) => {
      setLogs((prevLogs) => [...prevLogs, event.payload]);
    });

    // Cleanup the listener on component unmount
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <main className="p-4">
      <h1 className="mb-3 mb-4 text-2xl text-[var(--moss-primary)]">Logs</h1>
      <div className="rounded bg-gray-100 p-4">
        {logs.length > 0 ? (
          logs.map((log, index) => (
            <p key={index} className="text-[var(--moss-primary)]">
              {log}
            </p>
          ))
        ) : (
          <p className="text-secondary">No logs received yet...</p>
        )}
      </div>
    </main>
  );
};
