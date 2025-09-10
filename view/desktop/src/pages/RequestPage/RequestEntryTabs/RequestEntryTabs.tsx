import { useMemo, useState } from "react";

import { PageContainerWithTabs, TabItem } from "@/components";
import { Icon } from "@/lib/ui";
import { IDockviewPanelProps } from "@repo/moss-tabs";

import { useRequestPage } from "../hooks/useRequestPage";
import { RequestPageProps } from "../RequestPage";
import {
  AuthTabContent,
  BodyTabContent,
  HeadersTabContent,
  ParamsTabContent,
  PostRequestTabContent,
  PreRequestTabContent,
} from "../tabs";

export const RequestEntryTabs = ({ ...props }: IDockviewPanelProps<RequestPageProps>) => {
  const [activeRequestTabId, setActiveRequestTabId] = useState("params");
  const { requestData } = useRequestPage();

  const paramsCount = useMemo(() => {
    const queryParamsCount = requestData.url.query_params.filter(
      (param) => (param.key.trim() !== "" || param.value.trim() !== "") && !param.disabled
    ).length;
    const pathParamsCount = requestData.url.path_params.filter(
      (param) => param.key.trim() !== "" && !param.disabled
    ).length;
    return queryParamsCount + pathParamsCount;
  }, [requestData.url.query_params, requestData.url.path_params]);

  const requestTabs: TabItem[] = [
    {
      id: "params",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="SquareBrackets" className="h-4 w-4" />
          <span>Params</span>
          <Badge count={paramsCount} />
        </div>
      ),
      content: <ParamsTabContent {...props} />,
    },
    {
      id: "auth",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="Auth" className="h-4 w-4" />
          <span>Auth</span>
        </div>
      ),
      content: <AuthTabContent {...props} />,
    },
    {
      id: "headers",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="Headers" className="h-4 w-4" />
          <span>Headers</span>
          <Badge count={3} />
        </div>
      ),
      content: <HeadersTabContent {...props} />,
    },
    {
      id: "body",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="Braces" className="h-4 w-4" />
          <span>Body</span>
        </div>
      ),
      content: <BodyTabContent {...props} />,
    },
    {
      id: "pre-request",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="PreRequest" className="h-4 w-4" />
          <span>Pre Request</span>
        </div>
      ),
      content: <PreRequestTabContent {...props} />,
    },
    {
      id: "post-request",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="PostRequest" className="h-4 w-4" />
          <span>Post Request</span>
        </div>
      ),
      content: <PostRequestTabContent {...props} />,
    },
  ];

  return (
    <div>
      <PageContainerWithTabs tabs={requestTabs} activeTabId={activeRequestTabId} onTabChange={setActiveRequestTabId} />
    </div>
  );
};
const Badge = ({ count }: { count: number }) => (
  <span className="background-(--moss-tab-badge-color) inline-flex h-3.5 w-3.5 min-w-[14px] items-center justify-center rounded-full text-xs leading-none text-(--moss-tab-badge-text)">
    <span className="relative top-[0.5px]">{count}</span>
  </span>
);
