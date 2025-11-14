import { useState } from "react";

import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { ExecuteVcsOperationInput, ExecuteVcsOperationOutput } from "@repo/moss-project";
import { EntryChange, ListChangesOutput } from "@repo/moss-workspace";

const GitTest = () => {
  const [resourceChanges, setResourceChanges] = useState<EntryChange[]>([]);
  const [selectedKeys, setSelectedKeys] = useState<Record<string, boolean>>({});
  const [targetProjectId, setTargetProjectId] = useState<string>("");
  const [push, setPush] = useState(false);

  const keyFor = (c: EntryChange) => `${c.projectId}:${c.path}`;

  async function handleFileStatusesButton() {
    const result = await invokeTauriIpc<ListChangesOutput>("list_changes", {});

    if (result.status === "error") {
      throw new Error(String(result.status));
    }

    setResourceChanges(result.data.changes);
  }

  function toggleSelection(key: string) {
    setSelectedKeys((prev) => ({ ...prev, [key]: !prev[key] }));
  }

  function togglePush() {
    setPush(!push);
  }

  async function handleCommitButton() {
    const selected = resourceChanges.filter((ch) => selectedKeys[keyFor(ch)]);

    const grouped: Record<string, EntryChange[]> = {};
    for (const ch of selected) {
      if (!grouped[ch.projectId]) grouped[ch.projectId] = [];
      grouped[ch.projectId].push(ch);
    }

    for (const projectId in grouped) {
      const input: ExecuteVcsOperationInput = {
        operation: {
          "COMMIT": {
            message: `Committed from app at ${Date.now()}`,
            paths: grouped[projectId].map((ch) => ch.path),
            push: push,
          },
        },
      };
      const result = await invokeTauriIpc<ExecuteVcsOperationOutput>("execute_vcs_operation", {
        projectId: projectId,
        input: input,
      });
      if (result.status === "error") {
        throw new Error(String(result.status));
      }
    }
  }

  async function handleDiscardButton() {
    const selected = resourceChanges.filter((ch) => selectedKeys[keyFor(ch)]);

    const grouped: Record<string, EntryChange[]> = {};
    for (const ch of selected) {
      if (!grouped[ch.projectId]) grouped[ch.projectId] = [];
      grouped[ch.projectId].push(ch);
    }

    for (const projectId in grouped) {
      const input: ExecuteVcsOperationInput = {
        operation: {
          "DISCARD": {
            paths: grouped[projectId].map((ch) => ch.path),
          },
        },
      };
      const result = await invokeTauriIpc<ExecuteVcsOperationOutput>("execute_vcs_operation", {
        projectId: projectId,
        input: input,
      });
      if (result.status === "error") {
        throw new Error(String(result.status));
      }
    }
  }

  async function handleFetchButton() {
    const input: ExecuteVcsOperationInput = {
      operation: "FETCH",
    };
    const result = await invokeTauriIpc<ExecuteVcsOperationOutput>("execute_vcs_operation", {
      projectId: targetProjectId,
      input: input,
    });
    if (result.status === "error") {
      throw new Error(String(result.status));
    }
  }

  async function handlePullButton() {
    const input: ExecuteVcsOperationInput = {
      operation: "PULL",
    };
    const result = await invokeTauriIpc<ExecuteVcsOperationOutput>("execute_vcs_operation", {
      projectId: targetProjectId,
      input: input,
    });
    if (result.status === "error") {
      throw new Error(String(result.status));
    }
  }

  async function handlePushButton() {
    const input: ExecuteVcsOperationInput = {
      operation: "PUSH",
    };
    const result = await invokeTauriIpc<ExecuteVcsOperationOutput>("execute_vcs_operation", {
      projectId: targetProjectId,
      input: input,
    });
    if (result.status === "error") {
      throw new Error(String(result.status));
    }
  }

  const selectedCount = resourceChanges.reduce((acc, ch) => acc + (selectedKeys[keyFor(ch)] ? 1 : 0), 0);

  return (
    <>
      <div>
        <button
          className="cursor-pointer rounded bg-green-500 p-2 text-white hover:bg-green-600"
          onClick={handleFileStatusesButton}
        >
          Test File Statuses
        </button>
        <button
          className={`cursor-pointer rounded p-2 text-white ${selectedCount ? "bg-indigo-600 hover:bg-indigo-700" : "cursor-not-allowed bg-gray-400"}`}
          onClick={handleCommitButton}
          disabled={selectedCount === 0}
        >
          Commit Selected ({selectedCount})
        </button>
        <label>Push:</label>
        <input type="checkbox" checked={push} onChange={() => togglePush()} />
        <button
          className={`cursor-pointer rounded p-2 text-white ${selectedCount ? "bg-red-600 hover:bg-red-700" : "cursor-not-allowed bg-gray-400"}`}
          onClick={handleDiscardButton}
          disabled={selectedCount === 0}
        >
          Discard Selected ({selectedCount})
        </button>
      </div>

      <div>
        <input
          type="text"
          className="bg-white"
          placeholder="Collection Id"
          onChange={(e) => setTargetProjectId(e.target.value)}
          value={targetProjectId}
        />
        <button
          className="cursor-pointer rounded bg-pink-500 p-2 text-white hover:bg-pink-600"
          onClick={handleFetchButton}
        >
          Fetch Collection {`${targetProjectId}`}
        </button>
        <button
          className="cursor-pointer rounded bg-purple-500 p-2 text-white hover:bg-purple-600"
          onClick={handlePullButton}
        >
          Pull Collection {`${targetProjectId}`}
        </button>
        <button
          className="cursor-pointer rounded bg-yellow-500 p-2 text-white hover:bg-yellow-600"
          onClick={handlePushButton}
        >
          Push Collection {`${targetProjectId}`}
        </button>
      </div>

      <div className="max-h-64 overflow-auto rounded border p-2">
        {resourceChanges.length === 0 ? (
          <div className="text-sm text-gray-500">No changes loaded. Click "Test File Statuses".</div>
        ) : (
          <ul className="space-y-2">
            {resourceChanges.map((ch) => {
              const k = keyFor(ch);
              return (
                <li key={k} className="flex items-start gap-3">
                  <input
                    id={`chk-${k}`}
                    type="checkbox"
                    checked={!!selectedKeys[k]}
                    onChange={() => toggleSelection(k)}
                    className="mt-1"
                  />
                  <label htmlFor={`chk-${k}`} className="flex-1">
                    <div className="text-sm font-medium">{ch.path}</div>
                    <div className="text-xs text-gray-500">
                      Collection: {ch.projectId} — Status: {ch.status}
                    </div>
                  </label>
                </li>
              );
            })}
          </ul>
        )}
      </div>
    </>
  );
};

export default GitTest;
