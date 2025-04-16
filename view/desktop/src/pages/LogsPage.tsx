import React, { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useActivityEvents } from "@/context/ActivityEventsContext";
import { ActivityEventSimulator } from "@/components/ActivityEventSimulator";

export const Logs: React.FC = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  const [logs, setLogs] = useState<string[]>([]);
  const { activityEvents } = useActivityEvents();

  useEffect(() => {
    const unlistenLogsStream = listen<string>("logs-stream", (event) => {
      setLogs((prevLogs) => [...prevLogs, event.payload]);
    });

    return () => {
      unlistenLogsStream.then((unlisten) => unlisten());
    };
  }, []);

  const startIndexing = async () => {
    try {
      await invoke("example_index_collection_command");
      console.log("Indexing started");
    } catch (error) {
      console.error("Error starting indexing:", error);
    }
  };

  return (
    <main className="p-4">
      <h1 className="mb-4 text-2xl">{t("logs")}</h1>

      <ActivityEventSimulator className="mb-4" />

      <div className="mb-4 flex gap-2">
        <button onClick={startIndexing} className="cursor-pointer rounded bg-blue-500 p-2 text-white">
          {t("startIndexing")}
        </button>
      </div>

      <section className="mb-4">
        <h2 className="text-xl">{t("Last Progress Update")}</h2>

        {activityEvents.length > 0 ? (
          <ul>
            {activityEvents.map((activityEvent, index) => (
              <li key={index}>{JSON.stringify(activityEvent)}</li>
            ))}
          </ul>
        ) : (
          <p className="text-secondary">{t("noLogs")}...</p>
        )}
      </section>

      <section className="rounded bg-gray-100 p-4">
        <h2 className="mb-2 text-xl">{t("All Logs")}</h2>
        {logs.length > 0 ? (
          <ul>
            {logs.map((log, index) => (
              <li key={index}>{log}</li>
            ))}
          </ul>
        ) : (
          <p className="text-secondary">{t("noLogs")}...</p>
        )}
      </section>
    </main>
  );
};
