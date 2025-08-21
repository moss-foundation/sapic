import React from "react";

import { PageContainer } from "./PageContainer";
import { PageContainerHeader } from "./PageContainerHeader";
import {
  PageContainerTab,
  PageContainerTabContent,
  PageContainerTabs,
  PageContainerTabsList,
} from "./PageContainerTabs";
import { PageContainerWithTabsProps } from "./types";

export const PageContainerWithTabs: React.FC<PageContainerWithTabsProps> = ({
  tabs,
  activeTabId,
  onTabChange,
  className,
  noPadding,
}) => {
  return (
    <PageContainer className={className}>
      <PageContainerTabs value={activeTabId} onValueChange={onTabChange}>
        <PageContainerHeader className="border-b border-(--moss-border-color)">
          <PageContainerTabsList className="px-5">
            {tabs.map((tab) => (
              <PageContainerTab key={tab.id} value={tab.id}>
                {tab.icon && <div className="flex-shrink-0">{tab.icon}</div>}
                <span>{tab.label}</span>
              </PageContainerTab>
            ))}
          </PageContainerTabsList>
        </PageContainerHeader>

        {tabs.map((tab) => (
          <PageContainerTabContent key={`content-${tab.id}`} value={tab.id} noPadding={noPadding}>
            {tab.content}
          </PageContainerTabContent>
        ))}
      </PageContainerTabs>
    </PageContainer>
  );
};
