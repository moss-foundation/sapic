import { resourceDetailsCollection } from "@/db/resourceDetailsCollection";
import { PageView } from "@/workbench/ui/components";
import { PageWrapper } from "@/workbench/ui/components/PageView/PageWrapper";
import { DefaultViewProps } from "@/workbench/ui/parts/TabbedPane/types";
import { eq, useLiveQuery } from "@tanstack/react-db";

import { EndpointViewBody, EndpointViewHeader } from "./components";
import { EndpointViewContext } from "./EndpointViewContext";
import { useSyncResourceDetails } from "./hooks/useSyncResourceDetails";

export type EndpointViewProps = DefaultViewProps<{
  resourceId: string;
  projectId: string;
}>;

const EndpointView = ({ ...props }: EndpointViewProps) => {
  const { data: localResourceDetails } = useLiveQuery((q) =>
    q
      .from({ collection: resourceDetailsCollection })
      .where(({ collection }) => eq(collection.id, props.params.resourceId))
      .findOne()
  );

  useSyncResourceDetails({ resourceId: props.params.resourceId, projectId: props.params.projectId });

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
        projectId: props.params.projectId,
        resourceId: props.params.resourceId,
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
