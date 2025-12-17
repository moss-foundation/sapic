import { IDockviewPanelProps } from "moss-tabs";

import { resourcesDescriptionsCollection } from "@/app/resourcesDescriptionsCollection";
import { PageView } from "@/workbench/ui/components";
import { PageWrapper } from "@/workbench/ui/components/PageView/PageWrapper";
import { ResourceKind } from "@repo/moss-project";
import { eq, useLiveQuery } from "@tanstack/react-db";

import { EndpointViewBody, EndpointViewHeader } from "./components";
import { EndpointViewContext } from "./EndpointViewContext";
import { useSyncResourceDescriptionModel } from "./hooks/useSyncResourceDescriptionModel";

export interface EndpointViewProps {
  resourceId: string;
  projectId: string;
  //TODO since IconType is not used here and is needed in tabbed pane for the tab icon, we should consider removing it from the props here and add it to the tabbed pane props
  iconType: ResourceKind;
}

const EndpointView = ({ ...props }: IDockviewPanelProps<EndpointViewProps>) => {
  const { data: localResourceDescription } = useLiveQuery((q) =>
    q
      .from({ collection: resourcesDescriptionsCollection })
      .where(({ collection }) => eq(collection.id, props.params.resourceId))
      .findOne()
  );

  useSyncResourceDescriptionModel({ resourceId: props.params.resourceId, projectId: props.params.projectId });

  if (!localResourceDescription) {
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
