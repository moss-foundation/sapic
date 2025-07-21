import React from "react";

import { ActionButton, Breadcrumbs, PageContent, PageHeader, PageTabs, PageToolbar, PageView } from "@/components";
import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { Icon } from "@/lib/ui";
import { useRequestModeStore } from "@/store/requestMode";
import { cn } from "@/utils";
import { EntryKind } from "@repo/moss-collection";
import { IDockviewPanelProps } from "@repo/moss-tabs";

import Metadata from "../../parts/TabbedPane/DebugComponents/Metadata";
import { RequestInputField } from "./RequestInputField";

const DebugContext = React.createContext<boolean>(false);

interface RequestPageProps
  extends IDockviewPanelProps<{
    node?: TreeCollectionNode;
    treeId: string;
    iconType: EntryKind;
    someRandomString: string;
  }> {}

const RequestPage: React.FC<RequestPageProps> = (props) => {
  const { displayMode } = useRequestModeStore();

  const isDebug = React.useContext(DebugContext);

  let showEndpoint = false;
  let dontShowTabs = true;
  const [activeTab, setActiveTab] = React.useState(showEndpoint ? "endpoint" : "request");

  if (props.params?.node) {
    showEndpoint = displayMode === "DESIGN_FIRST" && props.params.node.class === "Endpoint";
    dontShowTabs =
      props.params.node.kind === "Dir" ||
      props.params.node.class === "Endpoint" ||
      props.params.node.class === "Schema";
  }

  const tabs = (
    <PageTabs>
      {showEndpoint && (
        <button data-active={activeTab === "endpoint"} onClick={() => setActiveTab("endpoint")}>
          Endpoint
        </button>
      )}
      <button data-active={activeTab === "request"} onClick={() => setActiveTab("request")}>
        Request
      </button>
      <button data-active={activeTab === "mock"} onClick={() => setActiveTab("mock")}>
        Mock
      </button>
    </PageTabs>
  );

  const toolbar = (
    <PageToolbar>
      <ActionButton icon="MoreHorizontal" />
    </PageToolbar>
  );

  const handleSendRequest = (method: string, url: string) => {
    console.log("Sending request:", { method, url });
    // TODO: Implement actual request sending logic
  };

  return (
    <PageView>
      <PageHeader
        icon={<Icon icon="Placeholder" className="size-[18px]" />}
        tabs={dontShowTabs ? null : tabs}
        toolbar={toolbar}
        props={props}
      />
      <PageContent className={cn("relative", isDebug && "border-2 border-dashed border-orange-500")}>
        <Breadcrumbs panelId={props.api.id} />

        <div className="flex h-full flex-col p-4">
          {props.params?.node ? (
            <div className="flex-1">
              {/* Request Input Section */}
              <div className="mb-6">
                <RequestInputField
                  initialMethod="POST"
                  initialUrl="{'{{baseUrl}}/docs/:docId/tables/:tableIdOrName/columns'}"
                  onSend={handleSendRequest}
                  className="mb-4"
                />

                {/* Request Configuration Tabs */}
                <div className="flex border-b border-gray-200">
                  <button className="border-b-2 border-blue-600 px-4 py-2 text-sm font-medium text-blue-600">
                    Params
                    <span className="ml-2 rounded-full bg-blue-100 px-2 py-0.5 text-xs text-blue-600">6</span>
                  </button>
                  <button className="px-4 py-2 text-sm font-medium text-gray-500 hover:text-gray-700">Auth</button>
                  <button className="px-4 py-2 text-sm font-medium text-gray-500 hover:text-gray-700">
                    Headers
                    <span className="ml-2 rounded-full bg-gray-100 px-2 py-0.5 text-xs text-gray-600">3</span>
                  </button>
                  <button className="px-4 py-2 text-sm font-medium text-gray-500 hover:text-gray-700">Body</button>
                  <button className="px-4 py-2 text-sm font-medium text-gray-500 hover:text-gray-700">
                    Pre Request
                  </button>
                  <button className="px-4 py-2 text-sm font-medium text-gray-500 hover:text-gray-700">
                    Post Request
                  </button>
                </div>

                {/* Parameters Section */}
                <div className="mt-4">
                  {/* Query Params */}
                  <div className="mb-6">
                    <h3 className="mb-3 text-sm font-medium text-gray-900">Query Params</h3>
                    <div className="overflow-hidden rounded-sm border border-gray-200 bg-white">
                      <table className="w-full">
                        <thead className="bg-gray-50">
                          <tr>
                            <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">
                              Key
                            </th>
                            <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">
                              Value
                            </th>
                            <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">
                              Type
                            </th>
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
                              <span className="text-sm text-gray-500">
                                An opaque token used to fetch the next page of results.
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
                              <span className="text-sm text-gray-500">
                                Maximum number of results to return in this query.
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
                              <input
                                type="text"
                                placeholder="Key"
                                className="w-full border-none text-sm outline-none"
                              />
                            </td>
                            <td className="px-3 py-2">
                              <input
                                type="text"
                                placeholder="Value"
                                className="w-full border-none text-sm outline-none"
                              />
                            </td>
                            <td className="px-3 py-2">
                              <select className="border-none bg-transparent text-sm outline-none">
                                <option>string</option>
                                <option>number</option>
                                <option>bool</option>
                              </select>
                            </td>
                            <td className="px-3 py-2">
                              <input
                                type="text"
                                placeholder="Description"
                                className="w-full border-none text-sm outline-none"
                              />
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
                            <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">
                              Key
                            </th>
                            <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">
                              Value
                            </th>
                            <th className="px-3 py-2 text-left text-xs font-medium tracking-wider text-gray-500 uppercase">
                              Type
                            </th>
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
                              <span className="text-sm text-gray-500">
                                An opaque token used to fetch the next page of results.
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
                              <span className="text-sm text-gray-500">
                                Maximum number of results to return in this query.
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
                              <input
                                type="text"
                                placeholder="Key"
                                className="w-full border-none text-sm outline-none"
                              />
                            </td>
                            <td className="px-3 py-2">
                              <input
                                type="text"
                                placeholder="Value"
                                className="w-full border-none text-sm outline-none"
                              />
                            </td>
                            <td className="px-3 py-2">
                              <select className="border-none bg-transparent text-sm outline-none">
                                <option>string</option>
                                <option>number</option>
                                <option>bool</option>
                              </select>
                            </td>
                            <td className="px-3 py-2">
                              <input
                                type="text"
                                placeholder="Description"
                                className="w-full border-none text-sm outline-none"
                              />
                            </td>
                            <td className="px-3 py-2"></td>
                          </tr>
                        </tbody>
                      </table>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          ) : (
            <div className="flex flex-1 items-center justify-center">
              <div className="text-center">
                <p className="mb-4 text-sm text-(--moss-secondary-text)">No request selected</p>
                {props?.params.someRandomString && (
                  <p className="text-xs text-(--moss-secondary-text)">Backend ID: {props.params.someRandomString}</p>
                )}
              </div>
            </div>
          )}

          {isDebug && (
            <Metadata
              onClick={() => {
                props.api.setRenderer(props.api.renderer === "always" ? "onlyWhenVisible" : "always");
              }}
              api={props.api}
            />
          )}
        </div>
      </PageContent>
    </PageView>
  );
};

export { RequestPage };
