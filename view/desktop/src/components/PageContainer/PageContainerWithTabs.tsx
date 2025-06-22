import React from "react";
import { PageContainer } from "./PageContainer";
import { PageContainerHeader } from "./PageContainerHeader";
import {
  PageContainerTabs,
  PageContainerTabsList,
  PageContainerTab,
  PageContainerTabContent,
} from "./PageContainerTabs";
import { PageContainerToolbar } from "./PageContainerToolbar";
import { PageContainerWithTabsProps } from "./types";

export const PageContainerWithTabs: React.FC<PageContainerWithTabsProps> = ({
  title,
  icon,
  tabs,
  activeTabId,
  onTabChange,
  toolbar,
  className,
}) => {
  const toolbarElement = toolbar ? <PageContainerToolbar>{toolbar}</PageContainerToolbar> : undefined;

  return (
    <PageContainer className={className}>
      <PageContainerTabs value={activeTabId} onValueChange={onTabChange}>
        {/* Header with tabs - matching PageView header structure */}
        <PageContainerHeader className="h-9 border-b border-(--moss-border-color)" toolbar={toolbarElement}>
          <div className="flex h-full items-center">
            {/* Title section - only render if title is provided */}
            {title && (
              <div className="flex min-w-0 flex-shrink-0 items-center gap-1.5 px-3">
                {icon && <div className="flex-shrink-0">{icon}</div>}
                <h1 className="truncate text-[16px] font-semibold text-(--moss-primary-text)">{title}</h1>
              </div>
            )}

            {/* Tabs section - using TabsList for proper Radix UI structure */}
            <div className="flex items-center px-3">
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
