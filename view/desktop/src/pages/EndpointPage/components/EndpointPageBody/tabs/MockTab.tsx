import { IDockviewPanelProps } from "@/lib/moss-tabs/src";
import { EndpointPageProps } from "@/pages/EndpointPage/EndpointPage";

export const MockTab = ({ ...props }: IDockviewPanelProps<EndpointPageProps>) => {
  return <div>MockTab</div>;
};
