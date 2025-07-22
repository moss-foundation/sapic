import React from "react";

import { Icon } from "@/lib/ui";
import { IDockviewPanelProps } from "@repo/moss-tabs";

export const ParamsTabContent = ({}: IDockviewPanelProps<{
  node?: any;
  treeId: string;
  iconType: any;
  someRandomString: string;
}>) => {
  return (
    <div className="mt-4">
      {/* Query Params */}
      <div className="mb-6">
        <h3 className="mb-3 text-sm font-medium text-gray-900">Query Params</h3>
        <div className="overflow-hidden rounded-sm border border-gray-200 bg-white">
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">Key</th>
                <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">
                  Value
                </th>
                <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">Type</th>
                <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">
                  Description
                </th>
                <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200">
              <tr>
                <td className="px-3 py-2">
                  <div className="flex items-center">
                    <input type="checkbox" className="mr-2" defaultChecked />
                    <span className="text-sm text-gray-900">pageToken</span>
                  </div>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-900">{"{{mu_func()}}"}</span>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-500">string</span>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-500">An opaque token used to fetch the next page of results.</span>
                </td>
                <td className="px-3 py-2">
                  <div className="flex items-center gap-1">
                    <button className="p-1 text-gray-400 hover:text-gray-600">
                      <Icon icon="Add" className="h-3 w-3" />
                    </button>
                    <button className="p-1 text-gray-400 hover:text-gray-600">
                      <Icon icon="Find" className="h-3 w-3" />
                    </button>
                    <button className="p-1 text-red-400 hover:text-red-600">
                      <Icon icon="Delete" className="h-3 w-3" />
                    </button>
                  </div>
                </td>
              </tr>
              <tr>
                <td className="px-3 py-2">
                  <div className="flex items-center">
                    <input type="checkbox" className="mr-2" defaultChecked />
                    <span className="text-sm text-gray-900">limit</span>
                  </div>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-900">{"{{defaultLimit}}"}</span>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-500">number</span>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-500">Maximum number of results to return in this query.</span>
                </td>
                <td className="px-3 py-2">
                  <div className="flex items-center gap-1">
                    <button className="p-1 text-gray-400 hover:text-gray-600">
                      <Icon icon="Add" className="h-3 w-3" />
                    </button>
                    <button className="p-1 text-gray-400 hover:text-gray-600">
                      <Icon icon="Find" className="h-3 w-3" />
                    </button>
                    <button className="p-1 text-red-400 hover:text-red-600">
                      <Icon icon="Delete" className="h-3 w-3" />
                    </button>
                  </div>
                </td>
              </tr>
              <tr>
                <td className="px-3 py-2">
                  <div className="flex items-center">
                    <input type="checkbox" className="mr-2" />
                    <span className="text-sm text-gray-900">visibleOnly</span>
                  </div>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-900">true</span>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-500">bool</span>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-500">
                    If true, returns only visible columns for the table. This...
                  </span>
                </td>
                <td className="px-3 py-2">
                  <div className="flex items-center gap-1">
                    <button className="p-1 text-gray-400 hover:text-gray-600">
                      <Icon icon="Add" className="h-3 w-3" />
                    </button>
                    <button className="p-1 text-gray-400 hover:text-gray-600">
                      <Icon icon="Find" className="h-3 w-3" />
                    </button>
                    <button className="p-1 text-red-400 hover:text-red-600">
                      <Icon icon="Delete" className="h-3 w-3" />
                    </button>
                  </div>
                </td>
              </tr>
              {/* Add new parameter row */}
              <tr>
                <td className="px-3 py-2">
                  <input type="text" placeholder="Key" className="w-full border-none text-sm outline-none" />
                </td>
                <td className="px-3 py-2">
                  <input type="text" placeholder="Value" className="w-full border-none text-sm outline-none" />
                </td>
                <td className="px-3 py-2">
                  <select className="border-none bg-transparent text-sm outline-none">
                    <option>string</option>
                    <option>number</option>
                    <option>bool</option>
                  </select>
                </td>
                <td className="px-3 py-2">
                  <input type="text" placeholder="Description" className="w-full border-none text-sm outline-none" />
                </td>
                <td className="px-3 py-2"></td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      {/* Path Params */}
      <div>
        <h3 className="mb-3 text-sm font-medium text-gray-900">Path Params</h3>
        <div className="overflow-hidden rounded-sm border border-gray-200 bg-white">
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">Key</th>
                <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">
                  Value
                </th>
                <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">Type</th>
                <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">
                  Description
                </th>
                <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200">
              <tr>
                <td className="px-3 py-2">
                  <div className="flex items-center">
                    <input type="checkbox" className="mr-2" defaultChecked />
                    <span className="text-sm text-gray-900">docId</span>
                  </div>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-900">{"{{vault::myVariable}}"}</span>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-500">string</span>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-500">An opaque token used to fetch the next page of results.</span>
                </td>
                <td className="px-3 py-2">
                  <div className="flex items-center gap-1">
                    <button className="p-1 text-gray-400 hover:text-gray-600">
                      <Icon icon="Add" className="h-3 w-3" />
                    </button>
                    <button className="p-1 text-gray-400 hover:text-gray-600">
                      <Icon icon="Find" className="h-3 w-3" />
                    </button>
                    <button className="p-1 text-red-400 hover:text-red-600">
                      <Icon icon="Delete" className="h-3 w-3" />
                    </button>
                  </div>
                </td>
              </tr>
              <tr>
                <td className="px-3 py-2">
                  <div className="flex items-center">
                    <input type="checkbox" className="mr-2" defaultChecked />
                    <span className="text-sm text-gray-900">tableIdOrName</span>
                  </div>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-900">{"{{defaultLimit}}"}</span>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-500">number</span>
                </td>
                <td className="px-3 py-2">
                  <span className="text-sm text-gray-500">Maximum number of results to return in this query.</span>
                </td>
                <td className="px-3 py-2">
                  <div className="flex items-center gap-1">
                    <button className="p-1 text-gray-400 hover:text-gray-600">
                      <Icon icon="Add" className="h-3 w-3" />
                    </button>
                    <button className="p-1 text-gray-400 hover:text-gray-600">
                      <Icon icon="Find" className="h-3 w-3" />
                    </button>
                    <button className="p-1 text-red-400 hover:text-red-600">
                      <Icon icon="Delete" className="h-3 w-3" />
                    </button>
                  </div>
                </td>
              </tr>
              {/* Add new parameter row */}
              <tr>
                <td className="px-3 py-2">
                  <input type="text" placeholder="Key" className="w-full border-none text-sm outline-none" />
                </td>
                <td className="px-3 py-2">
                  <input type="text" placeholder="Value" className="w-full border-none text-sm outline-none" />
                </td>
                <td className="px-3 py-2">
                  <select className="border-none bg-transparent text-sm outline-none">
                    <option>string</option>
                    <option>number</option>
                    <option>bool</option>
                  </select>
                </td>
                <td className="px-3 py-2">
                  <input type="text" placeholder="Description" className="w-full border-none text-sm outline-none" />
                </td>
                <td className="px-3 py-2"></td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};
