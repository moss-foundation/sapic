import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import LangchainAgent from "@/ai/LangchainAgent";
import { ActivityEventSimulator } from "@/components/ActivityEventSimulator";
import { useActivityEvents } from "@/context/ActivityEventsContext";
import { LogEntry, LOGGING_SERVICE_CHANNEL } from "@repo/moss-logging";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export const Logs = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const { activityEvents } = useActivityEvents();

  useEffect(() => {
    const unlistenLogsStream = listen<LogEntry>(LOGGING_SERVICE_CHANNEL, (event) => {
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

      <section className="mb-6">
        <h2 className="mb-2 text-xl">AI Assistant</h2>
        <div className="rounded bg-gray-50 p-4">
          <LangchainAgent />
        </div>
      </section>

      <ActivityEventSimulator className="mb-4" />

      <div className="mb-4 flex gap-2">
        <button onClick={startIndexing} className="cursor-pointer rounded bg-blue-500 p-2 text-white">
          {t("startIndexing")}
        </button>
      </div>

      <section className="mb-4">
        <h2 className="text-xl">{t("Activity Events")}</h2>

        {activityEvents.length > 0 ? (
          <ul className="mt-2 space-y-1 rounded bg-gray-50 p-3">
            {activityEvents.map((activityEvent, index) => {
              // Format the event differently based on type
              let eventInfo;
              if ("start" in activityEvent) {
                eventInfo = (
                  <div className="flex items-center gap-2">
                    <span className="rounded bg-blue-100 px-2 py-0.5 text-blue-800">Start</span>
                    <span className="font-medium">{activityEvent.start.title}</span>
                    <span className="text-sm text-gray-500">ID: {activityEvent.start.activityId}</span>
                  </div>
                );
              } else if ("progress" in activityEvent) {
                eventInfo = (
                  <div className="flex items-center gap-2">
                    <span className="rounded bg-green-100 px-2 py-0.5 text-green-800">Progress</span>
                    <span className="text-gray-700">{activityEvent.progress.detail}</span>
                    <span className="text-sm text-gray-500">ID: {activityEvent.progress.activityId}</span>
                  </div>
                );
              } else if ("finish" in activityEvent) {
                eventInfo = (
                  <div className="flex items-center gap-2">
                    <span className="rounded bg-purple-100 px-2 py-0.5 text-purple-800">Finish</span>
                    <span className="text-sm text-gray-500">ID: {activityEvent.finish.activityId}</span>
                  </div>
                );
              } else if ("oneshot" in activityEvent) {
                eventInfo = (
                  <div className="flex items-center gap-2">
                    <span className="rounded bg-amber-100 px-2 py-0.5 text-amber-800">Oneshot</span>
                    <span className="font-medium">{activityEvent.oneshot.title}</span>
                    <span>{activityEvent.oneshot.detail}</span>
                  </div>
                );
              } else {
                eventInfo = JSON.stringify(activityEvent);
              }

              return (
                <li key={index} className="border-b border-gray-100 pb-1">
                  {eventInfo}
                </li>
              );
            })}
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
              <li key={index}>
                {log.id} {log.timestamp} {log.level} {log.resource} {log.message}
              </li>
            ))}
          </ul>
        ) : (
          <p className="text-secondary">{t("noLogs")}...</p>
        )}
      </section>
    </main>
  );
};
