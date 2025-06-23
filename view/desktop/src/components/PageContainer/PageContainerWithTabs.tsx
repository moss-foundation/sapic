import React from "react";
import { PageContainer } from "./PageContainer";
import { PageContainerHeader } from "./PageContainerHeader";
import {
  PageContainerTabs,
  PageContainerTabsList,
  PageContainerTab,
  PageContainerTabContent,
} from "./PageContainerTabs";

import { PageContainerWithTabsProps } from "./types";

export const PageContainerWithTabs: React.FC<PageContainerWithTabsProps> = ({
  tabs,
  activeTabId,
  onTabChange,
  className,
}) => {
  return (
    <PageContainer className={className}>
      <PageContainerTabs value={activeTabId} onValueChange={onTabChange}>
        <PageContainerHeader className="h-9 border-b border-(--moss-border-color)">
          <div className="flex h-full items-center">
            {/* Tabs section - using TabsList for proper Radix UI structure */}
            <div className="flex items-center">
              <PageContainerTabsList>
                {tabs.map((tab) => (
                  <PageContainerTab key={tab.id} value={tab.id}>
                    {tab.icon && <div className="flex-shrink-0">{tab.icon}</div>}
                    <span>{tab.label}</span>
                  </PageContainerTab>
                ))}
              </PageContainerTabsList>
            </div>
          </div>
        </PageContainerHeader>

        {/* Tab content */}
        {tabs.map((tab) => (
          <PageContainerTabContent key={`content-${tab.id}`} value={tab.id}>
            {tab.content}
          </PageContainerTabContent>
        ))}
      </PageContainerTabs>
    </PageContainer>
  );
};
