import { useState } from "react";
import { invokeTauriIpc } from "@/lib/backend/tauri.ts";
import { ListChangesOutput } from "@repo/moss-workspace";

const FileStatuses = () => {
  const [fileStatuses, setFilesStatuses] = useState<string>("");

  async function handleFileStatusesButton() {
    const result = await invokeTauriIpc<ListChangesOutput>("list_changes", {});

    if (result.status === "error") {
      throw new Error(String(result.status));
    }

    setFilesStatuses(
      result.data.changes
        .map((change) => {
          return `Collection: ${change.collectionId}, Path: ${change.path}, Status: ${change.status}`;
        })
        .join("\n")
    );
  }

  return (
    <>
      <div>
        <textarea value={fileStatuses} readOnly className="w-full"></textarea>
      </div>
      <button
        className="cursor-pointer rounded bg-green-500 p-2 text-white hover:bg-green-600"
        onClick={handleFileStatusesButton}
      >
        Test File Statuses
      </button>
    </>
  );
};

export default FileStatuses;
