import { useMemo, useState } from "react";

import { PageContainerTabs, TabItemProps } from "@/components";
import { IDockviewPanelProps } from "@/lib/moss-tabs/src";

import { useRequestPage } from "../../../../../hooks/useRequestPage";
import { RequestPageProps } from "../../../../../RequestPage";
import {
  AuthTabContent,
  BodyTabContent,
  HeadersTabContent,
  ParamsTabContent,
  PostRequestTabContent,
  PreRequestTabContent,
} from "./tabs";

export const InputView = ({ ...props }: IDockviewPanelProps<RequestPageProps>) => {
  const [activeRequestTabId, setActiveRequestTabId] = useState("params");

  const {
    requestData,
    httpMethod: _httpMethod,
    setHttpMethod: _setHttpMethod,
    updateRequestData: _updateRequestData,
  } = useRequestPage();

  const paramsCount = useMemo(() => {
    const queryParamsCount = requestData.url.query_params.filter(
      (param) => (param.key.trim() !== "" || param.value.trim() !== "") && !param.disabled
    ).length;
    const pathParamsCount = requestData.url.path_params.filter(
      (param) => param.key.trim() !== "" && !param.disabled
    ).length;
    return queryParamsCount + pathParamsCount;
  }, [requestData.url.query_params, requestData.url.path_params]);

  const requestTabs: TabItemProps[] = [
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
      value={activeRequestTabId}
      onValueChange={setActiveRequestTabId}
      className="flex grow flex-col"
    >
      <PageContainerTabs.List>
        {requestTabs.map((tab) => (
          <PageContainerTabs.Trigger key={tab.id} value={tab.id} icon={tab.icon} count={tab.count}>
            {tab.label}
          </PageContainerTabs.Trigger>
        ))}
      </PageContainerTabs.List>

      {requestTabs.map((tab) => (
        <PageContainerTabs.Content key={tab.id} value={tab.id} className="flex grow">
          {tab.content}
        </PageContainerTabs.Content>
      ))}
    </PageContainerTabs.Root>
  );
};
