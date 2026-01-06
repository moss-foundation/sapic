import { useGetLocalResourceDetails } from "@/db/resource/hooks/useGetLocalResourceDetails";
import { PageView } from "@/workbench/ui/components";
import { PageWrapper } from "@/workbench/ui/components/PageView/PageWrapper";
import { DefaultViewProps } from "@/workbench/ui/parts/TabbedPane/types";

import { EndpointViewBody, EndpointViewHeader } from "./components";
import { EndpointViewContext } from "./EndpointViewContext";
import { useSyncResourceDetails } from "./hooks/useSyncResourceDetails";

export type EndpointViewProps = DefaultViewProps<{
  resourceId: string;
  projectId: string;
}>;

const EndpointView = ({ params }: EndpointViewProps) => {
  const localResourceDetails = useGetLocalResourceDetails(params.resourceId);

  useSyncResourceDetails({ resourceId: params.resourceId, projectId: params.projectId });

  if (!localResourceDetails) {
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
    <EndpointViewContext.Provider
      value={{
        projectId: params.projectId,
        resourceId: params.resourceId,
      }}
    >
      <PageView>
        <EndpointViewHeader />
        <EndpointViewBody />
      </PageView>
    </EndpointViewContext.Provider>
  );
};

export { EndpointView };
