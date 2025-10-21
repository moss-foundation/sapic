import { IDockviewPanelProps } from "moss-tabs";

import { PageView } from "@/components";
import { PageWrapper } from "@/components/PageView/PageWrapper";
import { ProjectTreeNode } from "@/components/ProjectTree/types";
import { useDescribeProjectResource, useStreamProjectResources } from "@/hooks";
import { ResourceKind } from "@repo/moss-project";

import { EndpointPageBody, EndpointPageHeader } from "./components";
import { EndpointPageContext } from "./EndpointPageContext";

export interface EndpointPageProps {
  node: ProjectTreeNode;
  projectId: string;
  iconType: ResourceKind;
}

const EndpointPage = ({ ...props }: IDockviewPanelProps<EndpointPageProps>) => {
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
            <p className="mb-4 text-sm text-(--moss-secondary-foreground)">No endpoint selected</p>
          </div>
        </div>
      </PageWrapper>
    );
  }

  return (
    <EndpointPageContext.Provider value={{ projectId: props.params.projectId, resourceDescription, resource }}>
      <PageView>
        <EndpointPageHeader />
        <EndpointPageBody />
      </PageView>
    </EndpointPageContext.Provider>
  );
};

export { EndpointPage };
