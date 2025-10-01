import { useMemo, useState } from "react";

import { PageContainerTabs, TabItemProps } from "@/components";
import { IDockviewPanelProps } from "@/lib/moss-tabs/src";

import { EndpointPageProps } from "../../../../../EndpointPage";
import { useEndpointPage } from "../../../../../hooks/useEndpointPage";
import {
  AuthTabContent,
  BodyTabContent,
  HeadersTabContent,
  ParamsTabContent,
  PostRequestTabContent,
  PreRequestTabContent,
} from "./tabs";

export const InputView = ({ ...props }: IDockviewPanelProps<EndpointPageProps>) => {
  const [activeEndpointTabId, setActiveEndpointTabId] = useState("params");

  const { endpointData } = useEndpointPage();

  const paramsCount = useMemo(() => {
    const queryParamsCount = endpointData.url.query_params.filter(
      (param) => (param.key.trim() !== "" || param.value.trim() !== "") && !param.disabled
    ).length;
    const pathParamsCount = endpointData.url.path_params.filter(
      (param) => param.key.trim() !== "" && !param.disabled
    ).length;
    return queryParamsCount + pathParamsCount;
  }, [endpointData.url.query_params, endpointData.url.path_params]);

  const endpointTabs: TabItemProps[] = [
    {
      id: "params",
      label: "Params",
      icon: "SquareBrackets",
      count: 6,
      content: <ParamsTabContent {...props} />,
    },
    {
      id: "auth",
      label: "Auth",
      icon: "Auth",
      content: <AuthTabContent {...props} />,
    },
    {
      id: "headers",
      label: "Headers",
      icon: "Headers",
      count: 3,
      content: <HeadersTabContent {...props} />,
    },
    {
      id: "body",
      label: "Body",
      icon: "Braces",
      content: <BodyTabContent {...props} />,
    },
    {
      id: "pre-request",
      label: "Pre Request",
      icon: "PreRequest",
      content: <PreRequestTabContent {...props} />,
    },
    {
      id: "post-request",
      label: "Post Request",
      icon: "PostRequest",
      content: <PostRequestTabContent {...props} />,
    },
  ];

  return (
    <PageContainerTabs.Root
      value={activeEndpointTabId}
      onValueChange={setActiveEndpointTabId}
      className="flex grow flex-col"
    >
      <PageContainerTabs.List>
        {endpointTabs.map((tab) => (
          <PageContainerTabs.Trigger key={tab.id} value={tab.id} icon={tab.icon} count={tab.count}>
            {tab.label}
          </PageContainerTabs.Trigger>
        ))}
      </PageContainerTabs.List>

      {endpointTabs.map((tab) => (
        <PageContainerTabs.Content key={tab.id} value={tab.id} className="flex grow">
          {tab.content}
        </PageContainerTabs.Content>
      ))}
    </PageContainerTabs.Root>
  );
};
