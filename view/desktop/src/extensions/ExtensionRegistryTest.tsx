import { useState } from "react";
import { invokeTauriIpc } from "@/lib/backend/tauri.ts";
import { AvailableExtensionInfo, ListAvailableExtensionsOutput } from "@repo/moss-app";

const ExtensionRegistryTest = () => {
  const [availableExtensions, setAvailableExtensions] = useState<AvailableExtensionInfo[]>([]);

  async function handleListExtensionsButton() {
    const result = await invokeTauriIpc<ListAvailableExtensionsOutput>("list_available_extensions", {});
    if (result.status === "error") {
      throw new Error(String(result.status));
    }

    setAvailableExtensions(result.data);
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
          {availableExtensions.map((info) => {
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

export default ExtensionRegistryTest;
