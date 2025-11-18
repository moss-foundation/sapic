import { IDockviewPanelProps } from "moss-tabs";
import { useState } from "react";

import { useStreamProjects } from "@/adapters/tanstackQuery/project";
import { FolderTabs, Icon, TabItemProps } from "@/lib/ui";
import { RoundedCounter } from "@/lib/ui/RoundedCounter";
import { useRenameProjectForm } from "@/workbench/hooks/useRenameProjectForm";
import { PageHeader, PageView } from "@/workbench/ui/components/PageView";
import { PageWrapper } from "@/workbench/ui/components/PageView/PageWrapper";

import { AuthTabContent } from "./tabs/AuthTabContent";
import { HeadersTabContent } from "./tabs/HeadersTabContent";
import { OverviewTabContent } from "./tabs/OverviewTabContent";
import { PostRequestTabContent } from "./tabs/PostRequestTabContent";
import { PreRequestTabContent } from "./tabs/PreRequestTabContent";
import { VariablesTabContent } from "./tabs/VariablesTabContent";

// Indicator dot for status
const PlaceholderStatusDot = ({ active }: { active: boolean }) =>
  active ? <div className="background-(--moss-green-4) h-2 w-2 rounded-full" /> : null;

export interface ProjectSettingsParams {
  projectId: string;
}

export const ProjectSettingsView = ({ ...props }: IDockviewPanelProps<ProjectSettingsParams>) => {
  const { projectId } = props.params;

  const { data: streamedProjects } = useStreamProjects();
  const project = streamedProjects?.find((p) => p.id === projectId);

  const [activeTabId, setActiveTabId] = useState("overview");

  const { isRenamingProject, setIsRenamingProject, handleRenamingProjectFormSubmit, handleRenamingProjectFormCancel } =
    useRenameProjectForm(projectId);

  if (!projectId) {
    return (
      <div className="text-(--moss-primary-foreground) flex h-full items-center justify-center">
        <div className="text-center">
          <h2 className="text-lg font-semibold">No Active Project</h2>
          <p className="text-sm">Please select a project to view its settings.</p>
        </div>
      </div>
    );
  }

  const tabs: TabItemProps[] = [
    {
      id: "overview",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="SquareBrackets" className="h-4 w-4" />
          <span>Overview</span>
        </div>
      ),
      content: <OverviewTabContent {...props} />,
    },
    {
      id: "auth",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="Auth" className="h-4 w-4" />
          <span>Auth</span>
          <PlaceholderStatusDot active={true} />
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
          <RoundedCounter count={3} color="primary" />
        </div>
      ),
      content: <HeadersTabContent {...props} />,
    },
    {
      id: "variables",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="Braces" className="h-4 w-4" />
          <span>Variables</span>
          <RoundedCounter count={3} color="primary" />
        </div>
      ),
      content: <VariablesTabContent {...props} />,
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
    <PageView>
      <PageHeader
        icon="Project"
        title={project?.name}
        disableTitleChange={false}
        onTitleChange={handleRenamingProjectFormSubmit}
        isRenamingTitle={isRenamingProject}
        setIsRenamingTitle={setIsRenamingProject}
        handleRenamingFormCancel={handleRenamingProjectFormCancel}
        {...props}
      />
      <PageWrapper>
        <FolderTabs.Root value={activeTabId} onValueChange={setActiveTabId}>
          <FolderTabs.List>
            {tabs.map((tab) => (
              <FolderTabs.Trigger key={tab.id} value={tab.id}>
                {tab.label}
              </FolderTabs.Trigger>
            ))}
          </FolderTabs.List>
          {tabs.map((tab) => (
            <FolderTabs.Content key={tab.id} value={tab.id} className="flex flex-1">
              <PageWrapper className="flex flex-1 flex-col">{tab.content}</PageWrapper>
            </FolderTabs.Content>
          ))}{" "}
        </FolderTabs.Root>
      </PageWrapper>
    </PageView>
  );
};
