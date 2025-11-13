import { IDockviewPanelProps } from "moss-tabs";

import { PageView } from "@/components";
import { PageWrapper } from "@/components/PageView/PageWrapper";
import { ProjectTreeNode } from "@/components/ProjectTree/types";
import { useDescribeProjectResource, useStreamProjectResources } from "@/hooks";
import { ResourceKind } from "@repo/moss-project";

import { EndpointViewBody, EndpointViewHeader } from "./components";
import { EndpointViewContext } from "./EndpointViewContext";

export interface EndpointViewProps {
  node: ProjectTreeNode;
  projectId: string;
  iconType: ResourceKind;
}

const EndpointView = ({ ...props }: IDockviewPanelProps<EndpointViewProps>) => {
  const { data: streamedResources } = useStreamProjectResources(props.params?.projectId);
  const resource = streamedResources?.find((resource) => resource.id === props.params?.node?.id);

  const { data: resourceDescription } = useDescribeProjectResource({
    projectId: props.params?.projectId ?? "",
    resourceId: resource?.id ?? "",
    options: {
      enabled: !!resource?.id,
    },
  });

  if (!resource || !resourceDescription) {
    return (
      <PageWrapper>
        <div className="flex flex-1 items-center justify-center">
          <div className="text-center">
            <p className="text-(--moss-secondary-foreground) mb-4 text-sm">No endpoint selected</p>
          </div>
        </div>
      </PageWrapper>
    );
  }

  return (
    <EndpointViewContext.Provider value={{ projectId: props.params.projectId, resourceDescription, resource }}>
      <PageView>
        <EndpointViewHeader />
        <EndpointViewBody />
      </PageView>
    </EndpointViewContext.Provider>
  );
};

export { EndpointView };
