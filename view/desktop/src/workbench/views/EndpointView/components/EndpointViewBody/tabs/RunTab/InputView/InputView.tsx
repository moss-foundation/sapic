import { useContext, useMemo, useState } from "react";

import { FolderTabs, TabItemProps } from "@/lib/ui";
import { EndpointViewContext } from "@/workbench/views/EndpointView/EndpointViewContext";

import {
  AuthTabContent,
  BodyTabContent,
  HeadersTabContent,
  ParamsTabContent,
  PostRequestTabContent,
  PreRequestTabContent,
} from "./tabs";

export const InputView = () => {
  const { resourceDescription } = useContext(EndpointViewContext);

  const [activeEndpointTabId, setActiveEndpointTabId] = useState("params");

  const numberOfActiveParams = useMemo(() => {
    const queryParamsCount = resourceDescription?.queryParams.filter((param) => !param.disabled).length ?? 0;
    const pathParamsCount = resourceDescription?.pathParams.filter((param) => !param.disabled).length ?? 0;

    return queryParamsCount + pathParamsCount;
  }, [resourceDescription?.queryParams, resourceDescription?.pathParams]);

  const endpointTabs: TabItemProps[] = [
    {
      id: "params",
      label: "Params",
      icon: "SquareBrackets",
      count: numberOfActiveParams,
      content: <ParamsTabContent />,
    },
    {
      id: "auth",
      label: "Auth",
      icon: "Auth",
      content: <AuthTabContent />,
    },
    {
      id: "headers",
      label: "Headers",
      icon: "Headers",
      count: 3,
      content: <HeadersTabContent />,
    },
    {
      id: "body",
      label: "Body",
      icon: "Braces",
      content: <BodyTabContent />,
    },
    {
      id: "pre-request",
      label: "Pre Request",
      icon: "PreRequest",
      content: <PreRequestTabContent />,
    },
    {
      id: "post-request",
      label: "Post Request",
      icon: "PostRequest",
      content: <PostRequestTabContent />,
    },
  ];

  return (
    <FolderTabs.Root value={activeEndpointTabId} onValueChange={setActiveEndpointTabId} className="flex grow flex-col">
      <FolderTabs.List>
        {endpointTabs.map((tab) => (
          <FolderTabs.Trigger key={tab.id} value={tab.id} icon={tab.icon} count={tab.count}>
            {tab.label}
          </FolderTabs.Trigger>
        ))}
      </FolderTabs.List>

      {endpointTabs.map((tab) => (
        <FolderTabs.Content key={tab.id} value={tab.id} className="flex grow">
          {tab.content}
        </FolderTabs.Content>
      ))}
    </FolderTabs.Root>
  );
};
