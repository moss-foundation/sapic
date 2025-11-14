import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import { useActivityRouter } from "@/hooks/app";
import { invokeTauriIpc } from "@/lib/backend/tauri.ts";
import { PageContent } from "@/workbench/ui/components";
import { ActivityEventSimulator } from "@/workbench/ui/components/ActivityEventSimulator";
import AIDemo from "@/workbench/ui/components/AIDemo";
import GitTest from "@/workbench/ui/components/GitTest";
import { ExtensionInfo } from "@repo/moss-extension";
import { AccountKind } from "@repo/moss-user";
import {
  AddAccountParams,
  ListExtensionsOutput,
  LogEntryInfo,
  ON_DID_APPEND_LOG_ENTRY_CHANNEL,
  UpdateProfileInput,
} from "@repo/window";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface CreateProfileData {
  name: string;
}

interface LoginData {
  profileId: string;
  accountId: string;
  provider: string;
}

export const LogsView = () => {
  const { t } = useTranslation(["main", "bootstrap"]);
  const [logs, setLogs] = useState<LogEntryInfo[]>([]);
  const { windowEvents } = useActivityRouter();

  const [profileForm, setProfileForm] = useState<CreateProfileData>({
    name: "",
  });

  const [accountForm, setAccountForm] = useState<AddAccountParams>({
    host: "github.com",
    kind: "GITHUB",
  });

  const [loginForm, setLoginForm] = useState<LoginData>({
    profileId: "",
    accountId: "",
    provider: "GitHub",
  });

  const [getItemForm, setGetItemForm] = useState({
    key: "",
    workspaceId: "",
  });

  const [putItemForm, setPutItemForm] = useState({
    key: "",
    workspaceId: "",
    value: "",
  });

  const [removeItemForm, setRemoveItemForm] = useState({
    key: "",
    workspaceId: "",
  });

  useEffect(() => {
    const unlistenLogsStream = listen<LogEntryInfo>(ON_DID_APPEND_LOG_ENTRY_CHANNEL, (event) => {
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
        accountsToUpdate: [],
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
      console.log(result);
    } catch (error) {
      console.error("Error describing app:", error);
    }
  };

  const handleGetItem = async () => {
    try {
      const result = await invokeTauriIpc("plugin:shared-storage|get_item", {
        input: {
          key: getItemForm.key,
          scope: getItemForm.workspaceId ? { workspace: getItemForm.workspaceId } : "application",
        },
      });
      console.log("Get item result:", result);
      if (result.status === "ok") {
        console.log("Data:", result.data);
      }
    } catch (error) {
      console.error("Error getting item:", error);
    }
  };

  const handlePutItem = async () => {
    try {
      const result = await invokeTauriIpc("plugin:shared-storage|put_item", {
        input: {
          key: putItemForm.key,
          scope: putItemForm.workspaceId ? { workspace: putItemForm.workspaceId } : "application",
          value: putItemForm.value,
        },
      });
      console.log("Put item result:", result);
      if (result.status === "ok") {
        console.log("Data:", result.data);
      }
    } catch (error) {
      console.error("Error putting item:", error);
    }
  };

  const handleRemoveItem = async () => {
    try {
      const result = await invokeTauriIpc("plugin:shared-storage|remove_item", {
        input: {
          key: removeItemForm.key,
          scope: removeItemForm.workspaceId ? { workspace: removeItemForm.workspaceId } : "application",
        },
      });
      console.log("Remove item result:", result);
      if (result.status === "ok") {
        console.log("Data:", result.data);
      }
    } catch (error) {
      console.error("Error removing item:", error);
    }
  };

  return (
    <PageContent className="space-y-6">
      <section className="mb-6">
        <h2 className="mb-2 text-xl">Extension Registry</h2>
        <div className="rounded bg-gray-50 p-4">
          <ExtensionRegistryTest />
        </div>
      </section>

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
        <h2 className="mb-2 text-xl">Shared Storage</h2>
        <div className="grid grid-cols-3 gap-4">
          <div className="flex flex-col gap-2 rounded bg-gray-50 p-4">
            <h3 className="text-lg font-medium">Get Item</h3>
            <input
              type="text"
              placeholder="Key"
              value={getItemForm.key}
              onChange={(e) => setGetItemForm((prev) => ({ ...prev, key: e.target.value }))}
              className="w-full rounded-md border border-gray-300 bg-white p-2"
            />
            <input
              type="text"
              placeholder="Workspace Id"
              value={getItemForm.workspaceId}
              onChange={(e) => setGetItemForm((prev) => ({ ...prev, workspaceId: e.target.value }))}
              className="w-full rounded-md border border-gray-300 bg-white p-2"
            />
            <button onClick={handleGetItem} className="w-full rounded bg-blue-500 p-2 text-white">
              Get
            </button>
          </div>

          <div className="flex flex-col gap-2 rounded bg-gray-50 p-4">
            <h3 className="text-lg font-medium">Put Item</h3>
            <input
              type="text"
              placeholder="Key"
              value={putItemForm.key}
              onChange={(e) => setPutItemForm((prev) => ({ ...prev, key: e.target.value }))}
              className="w-full rounded-md border border-gray-300 bg-white p-2"
            />
            <input
              type="text"
              placeholder="Workspace Id"
              value={putItemForm.workspaceId}
              onChange={(e) => setPutItemForm((prev) => ({ ...prev, workspaceId: e.target.value }))}
              className="w-full rounded-md border border-gray-300 bg-white p-2"
            />
            <input
              type="text"
              placeholder="Value"
              value={putItemForm.value}
              onChange={(e) => setPutItemForm((prev) => ({ ...prev, value: e.target.value }))}
              className="w-full rounded-md border border-gray-300 bg-white p-2"
            />
            <button onClick={handlePutItem} className="w-full rounded bg-blue-500 p-2 text-white">
              Put
            </button>
          </div>

          <div className="flex flex-col gap-2 rounded bg-gray-50 p-4">
            <h3 className="text-lg font-medium">Remove Item</h3>
            <input
              type="text"
              placeholder="Key"
              value={removeItemForm.key}
              onChange={(e) => setRemoveItemForm((prev) => ({ ...prev, key: e.target.value }))}
              className="w-full rounded-md border border-gray-300 bg-white p-2"
            />
            <input
              type="text"
              placeholder="Workspace Id"
              value={removeItemForm.workspaceId}
              onChange={(e) => setRemoveItemForm((prev) => ({ ...prev, workspaceId: e.target.value }))}
              className="w-full rounded-md border border-gray-300 bg-white p-2"
            />
            <button onClick={handleRemoveItem} className="w-full rounded bg-blue-500 p-2 text-white">
              Remove
            </button>
          </div>
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
                onChange={(e) => setAccountForm((prev) => ({ ...prev, host: e.target.value }))}
                className="w-full rounded-md border border-gray-300 bg-white p-2"
              />
              <select
                value={accountForm.kind}
                onChange={(e) => setAccountForm((prev) => ({ ...prev, kind: e.target.value as AccountKind }))}
                className="w-full rounded-md border border-gray-300 bg-white p-2"
              >
                <option value="github">GitHub</option>
                <option value="gitlab">GitLab</option>
              </select>
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

        {windowEvents.activityEvents.length > 0 ? (
          <ul className="mt-2 space-y-1 rounded bg-gray-50 p-3">
            {windowEvents.activityEvents.map((activityEvent, index) => {
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

const ExtensionRegistryTest = () => {
  const [extensions, setExtensions] = useState<ExtensionInfo[]>([]);

  async function handleListExtensionsButton() {
    const result = await invokeTauriIpc<ListExtensionsOutput>("list_extensions", {});
    if (result.status === "error") {
      throw new Error(String(result.status));
    }

    setExtensions(result.data);
  }

  return (
    <div className={"overflow-x-auto rounded-md"}>
      <table className={"min-w-full table-fixed divide-y divide-gray-200"}>
        <thead className={"bg-gray-50"}>
          <tr className={"text-left"}>
            <th className={"p-1"}>ID</th>
            <th className={"p-1"}>External ID</th>
            <th className={"p-1"}>Name</th>
            <th className={"p-1"}>Authors</th>
            <th className={"p-1"}>Description</th>
            <th className={"p-1"}>Repository</th>
            <th className={"p-1"}>Downloads</th>
            <th className={"p-1"}>Created At</th>
            <th className={"p-1"}>Updated At</th>
            <th className={"p-1"}>Latest Version</th>
          </tr>
        </thead>
        <tbody>
          {extensions.map((info) => {
            return (
              <tr>
                <td className={"p-1"}>{info.id}</td>
                <td className={"p-1"}>{info.externalId}</td>
                <td className={"p-1"}>{info.name}</td>
                <td className={"p-1"}>{info.authors}</td>
                <td className={"p-1"}>{info.description}</td>
                <td className={"p-1"}>{info.repository}</td>
                <td className={"p-1"}>{info.downloads.toString()}</td>
                <td className={"p-1"}>{info.createdAt}</td>
                <td className={"p-1"}>{info.updatedAt}</td>
                <td className={"p-1"}>{info.latestVersion}</td>
              </tr>
            );
          })}
        </tbody>
      </table>
      <button
        className="cursor-pointer rounded bg-purple-500 p-2 text-white hover:bg-purple-600"
        onClick={handleListExtensionsButton}
      >
        List Available Extensions From the Extension Registry
      </button>
    </div>
  );
};
