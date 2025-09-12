import { useState } from "react";

import { invokeTauriIpc } from "@/lib/backend/tauri.ts";
import { EntryChange, ListChangesOutput } from "@repo/moss-workspace";
import { ExecuteVcsOperationInput, ExecuteVcsOperationOutput } from "@repo/moss-collection";

const GitTest = () => {
  const [entryChanges, setEntryChanges] = useState<EntryChange[]>([]);
  const [selectedKeys, setSelectedKeys] = useState<Record<string, boolean>>({});
  const [push, setPush] = useState(false);

  const keyFor = (c: EntryChange) => `${c.collectionId}:${c.path}`;

  async function handleFileStatusesButton() {
    const result = await invokeTauriIpc<ListChangesOutput>("list_changes", {});

    if (result.status === "error") {
      throw new Error(String(result.status));
    }

    setEntryChanges(result.data.changes);
  }

  function toggleSelection(key: string) {
    setSelectedKeys((prev) => ({ ...prev, [key]: !prev[key] }));
  }

  function togglePush() {
    setPush(!push);
  }

  async function handleCommitButton() {
    const selected = entryChanges.filter((ch) => selectedKeys[keyFor(ch)]);

    const grouped: Record<string, EntryChange[]> = {};
    for (const ch of selected) {
      if (!grouped[ch.collectionId]) grouped[ch.collectionId] = [];
      grouped[ch.collectionId].push(ch);
    }

    for (const collectionId in grouped) {
      const input: ExecuteVcsOperationInput = {
        operation: {
          "COMMIT": {
            message: `Committed from app at ${Date.now()}`,
            paths: grouped[collectionId].map((ch) => ch.path),
            push: push,
          },
        },
      };
      const result = await invokeTauriIpc<ExecuteVcsOperationOutput>("execute_vcs_operation", {
        collectionId: collectionId,
        input: input,
      });
      if (result.status === "error") {
        throw new Error(String(result.status));
      }
    }
  }

  const selectedCount = entryChanges.reduce((acc, ch) => acc + (selectedKeys[keyFor(ch)] ? 1 : 0), 0);

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
          Collect Selected ({selectedCount})
        </button>
        <label>Push:</label>
        <input type="checkbox" checked={push} onChange={() => togglePush()} />
      </div>

      <div className="max-h-64 overflow-auto rounded border p-2">
        {entryChanges.length === 0 ? (
          <div className="text-sm text-gray-500">No changes loaded. Click "Test File Statuses".</div>
        ) : (
          <ul className="space-y-2">
            {entryChanges.map((ch) => {
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
                      Collection: {ch.collectionId} — Status: {ch.status}
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
