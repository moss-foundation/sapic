import React, { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

type ProgressStreamPayload = {
  collection_key: number;
  progress_percent: number;
  path: string;
};

export const Logs: React.FC = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  const [logs, setLogs] = useState<string[]>([]);
  const [progressEvents, setProgressEvents] = useState<ProgressStreamPayload[]>([]);

  useEffect(() => {
    const unlistenLogsStream = listen<string>("logs-stream", (event) => {
      setLogs((prevLogs) => [...prevLogs, event.payload]);
    });

    const unlistenProgressStream = listen<ProgressStreamPayload>("progress-stream", (event) => {
      console.log("progress-stream", event.payload.progress_percent);
      setProgressEvents((prev) => [...prev, event.payload]);
    });

    return () => {
      unlistenLogsStream.then((unlisten) => unlisten());
      unlistenProgressStream.then((unlisten) => unlisten());
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

  const lastProgressEvent = progressEvents[progressEvents.length - 1];

  return (
    <main className="p-4">
      <h1 className="mb-4 text-2xl">{t("logs")}</h1>
      <button onClick={startIndexing} className="mb-4 cursor-pointer rounded bg-blue-500 p-2 text-white">
        {t("startIndexing")}
      </button>

      <section className="mb-4">
        <h2 className="text-xl">{t("Last Progress Update")}</h2>
        {lastProgressEvent ? (
          <ul>
            <li>
              <strong>{t("Progress")}:</strong> {lastProgressEvent.progress_percent}%
            </li>
            <li>
              <strong>{t("Path")}:</strong> {lastProgressEvent.path}
            </li>
          </ul>
        ) : (
          <p>{t("No progress updates yet")}</p>
        )}
      </section>

      <section className="mb-4">
        <h2 className="text-xl">All Progress Events</h2>
        {progressEvents.length > 0 ? (
          <ul>
            {progressEvents.map((event, index) => (
              <li key={index}>
                {`Collection: ${event.collection_key}, Progress: ${event.progress_percent}%, Path: ${event.path}`}
              </li>
            ))}
          </ul>
        ) : (
          <p>{t("No progress events yet")}</p>
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
