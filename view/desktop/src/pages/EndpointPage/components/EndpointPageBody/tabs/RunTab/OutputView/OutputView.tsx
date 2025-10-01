import { useState } from "react";

import ActionButton from "@/components/ActionButton";
import { FolderTabs, TabItemProps } from "@/lib/ui";
import { IDockviewPanelProps } from "@repo/moss-tabs";

import { EndpointPageProps } from "../../../../../EndpointPage";
import { BodyTab, CookiesTab, HeadersTab } from "./tabs";

export const OutputView = ({ ...props }: IDockviewPanelProps<EndpointPageProps>) => {
  const [activeOutputTabId, setActiveOutputTabId] = useState("body");

  const outputTabs: TabItemProps[] = [
    {
      id: "body",
      label: "Body",
      icon: "Braces",
      content: <BodyTab />,
    },
    {
      id: "headers",
      label: "Headers",
      icon: "Headers",
      count: 3,
      content: <HeadersTab />,
    },
    {
      id: "cookies",
      label: "Cookies",
      icon: "Braces",
      content: <CookiesTab />,
    },
  ];
  return (
    <div className="flex flex-1 flex-col gap-3">
      <FolderTabs.Root value={activeOutputTabId} onValueChange={setActiveOutputTabId} className="flex grow flex-col">
        <FolderTabs.List toolbar={<ToolbarPlaceholder />}>
          {outputTabs.map((tab) => (
            <FolderTabs.Trigger key={tab.id} value={tab.id} icon={tab.icon} count={tab.count}>
              {tab.label}
            </FolderTabs.Trigger>
          ))}
        </FolderTabs.List>

        {outputTabs.map((tab) => (
          <FolderTabs.Content key={tab.id} value={tab.id} className="flex grow">
            {tab.content}
          </FolderTabs.Content>
        ))}
      </FolderTabs.Root>
    </div>
  );
};

const ToolbarPlaceholder = () => {
  return (
    <div className="flex items-center justify-between gap-5 text-sm">
      <div className="flex items-center gap-2">
        <div className="background-(--moss-green-11) rounded-md border border-(--moss-green-9) px-2 text-(--moss-success)">
          200 OK
        </div>
        <div className="background-(--moss-gray-8) size-1 rounded-full" />
        <div className="text-(--moss-gray-6)">1.24 ms</div>
        <div className="background-(--moss-gray-8) size-1 rounded-full" />
        <div className="text-(--moss-gray-6)">1.21 KB</div>
      </div>

      <div className="flex items-center">
        <button className="cursor-pointer px-2">Save Response</button>
        <ActionButton className="p-1.25" iconClassName="size-4.5" icon="MoreHorizontal" />
        <ActionButton className="p-1.25" iconClassName="size-4.5" icon="Broom" />
      </div>
    </div>
  );
};
