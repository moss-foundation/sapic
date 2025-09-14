import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import AIDemo from "@/ai/AIDemo.tsx";
import { PageContent } from "@/components";
import { ActivityEventSimulator } from "@/components/ActivityEventSimulator";
import { useActivityEvents } from "@/context/ActivityEventsContext";
import { AddAccountParams, LogEntryInfo, LOGGING_SERVICE_CHANNEL, UpdateProfileInput } from "@repo/moss-app";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import GitTest from "@/git/GitTest.tsx";

interface CreateProfileData {
  name: string;
}

interface LoginData {
  profileId: string;
  accountId: string;
  provider: string;
}

export const Logs = () => {
  const { t } = useTranslation(["ns1", "ns2"]);
  const [logs, setLogs] = useState<LogEntryInfo[]>([]);
  const { activityEvents } = useActivityEvents();

  const [profileForm, setProfileForm] = useState<CreateProfileData>({
    name: "",
  });

  const [accountForm, setAccountParams] = useState<AddAccountParams>({
    host: "github.com",
    label: "",
    kind: "GITHUB",
    pat: "",
  });

  const [loginForm, setLoginForm] = useState<LoginData>({
    profileId: "",
    accountId: "",
    provider: "GitHub",
  });

  useEffect(() => {
    const unlistenLogsStream = listen<LogEntryInfo>(LOGGING_SERVICE_CHANNEL, (event) => {
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

  const handleCreateProfile = async () => {
    try {
      await invoke("create_profile", {
        input: {
          name: profileForm.name,
        },
      });
      console.log("Profile created:", profileForm);
    } catch (error) {
      console.error("Error creating profile:", error);
    }
  };

  const handleAddAccount = async () => {
    try {
      const input: UpdateProfileInput = {
        accountsToAdd: [accountForm],
        accountsToRemove: [],
      };
      await invoke("update_profile", { input });
      console.log("Account added:", accountForm);
    } catch (error) {
      console.error("Error adding account:", error);
    }
  };

  const handleLogin = async () => {
    try {
      await invoke("user_login", {
        profileId: loginForm.profileId,
        provider: loginForm.provider,
      });
      console.log("Login successful:", loginForm);
    } catch (error) {
      console.error("Error logging in:", error);
    }
  };

  const handleDescribeApp = async () => {
    try {
      const result = await invoke("describe_app");
      console.log("App described:", result);
    } catch (error) {
      console.error("Error describing app:", error);
    }
  };

  return (
    <PageContent className="space-y-6">
      <section className="mb-6">
        <h2 className="mb-2 text-xl">File Statuses</h2>
        <div className="rounded bg-gray-50 p-4">
          <GitTest />
        </div>
      </section>
      <section className="mb-6">
        <h2 className="mb-2 text-xl">AI Assistant</h2>
        <div className="rounded bg-gray-50 p-4">
          <AIDemo />
        </div>
      </section>

      <section className="mb-6">
        <h2 className="mb-2 text-xl">App</h2>
        <div className="rounded bg-gray-50 p-4">
          <button onClick={handleDescribeApp} className="w-full rounded bg-blue-500 p-2 text-white">
            Describe App
          </button>
        </div>
      </section>

      <section className="mb-6">
        <h2 className="mb-2 text-xl">Profile</h2>
        <div className="rounded bg-gray-50 p-4">
          <div className="grid grid-cols-3 gap-4">
            <div className="flex flex-col gap-2">
              <h3 className="text-lg font-medium">Create profile</h3>
              <input
                type="text"
                placeholder="Profile name"
                value={profileForm.name}
                onChange={(e) => setProfileForm((prev) => ({ ...prev, name: e.target.value }))}
                className="w-full rounded-md border border-gray-300 bg-white p-2"
              />
              <button onClick={handleCreateProfile} className="w-full rounded bg-blue-500 p-2 text-white">
                Create
              </button>
            </div>

            <div className="flex flex-col gap-2">
              <h3 className="text-lg font-medium">Add account</h3>
              <input
                type="text"
                placeholder="Host"
                value={accountForm.host}
                onChange={(e) => setAccountParams((prev) => ({ ...prev, host: e.target.value }))}
                className="w-full rounded-md border border-gray-300 bg-white p-2"
              />
              <input
                type="text"
                placeholder="Label"
                value={accountForm.label}
                onChange={(e) => setAccountParams((prev) => ({ ...prev, label: e.target.value }))}
                className="w-full rounded-md border border-gray-300 bg-white p-2"
              />
              <select
                value={accountForm.kind}
                onChange={(e) => setAccountParams((prev) => ({ ...prev, provider: e.target.value.toUpperCase() }))}
                className="w-full rounded-md border border-gray-300 bg-white p-2"
              >
                <option value="github">GitHub</option>
                <option value="gitlab">GitLab</option>
              </select>
              <input
                type="text"
                placeholder=""
                value={accountForm.pat}
                onChange={(e) => {
                  const { pat: _pat, ...rest } = accountForm;
                  if (e.target.value == "") {
                    setAccountParams(rest);
                  } else {
                    setAccountParams({ ...rest, pat: e.target.value });
                  }
                }}
                className="w-full rounded-md border border-gray-300 bg-white p-2"
              />

              <button onClick={handleAddAccount} className="w-full rounded bg-blue-500 p-2 text-white">
                Add
              </button>
            </div>

            <div className="flex flex-col gap-2">
              <h3 className="text-lg font-medium">VCS Operations</h3>
              <input
                type="text"
                placeholder="Profile Id"
                value={loginForm.profileId}
                onChange={(e) => setLoginForm((prev) => ({ ...prev, profileId: e.target.value }))}
                className="w-full rounded-md border border-gray-300 bg-white p-2"
              />
              <input
                type="text"
                placeholder="Account Id"
                value={loginForm.accountId}
                onChange={(e) => setLoginForm((prev) => ({ ...prev, accountId: e.target.value }))}
                className="w-full rounded-md border border-gray-300 bg-white p-2"
              />
              <input
                type="text"
                placeholder="Account Id"
                value={loginForm.accountId}
                onChange={(e) => setLoginForm((prev) => ({ ...prev, accountId: e.target.value }))}
                className="w-full rounded-md border border-gray-300 bg-white p-2"
              />

              <button onClick={handleLogin} className="w-full rounded bg-blue-500 p-2 text-white">
                Login
              </button>
            </div>
          </div>
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
    </PageContent>
  );
};
