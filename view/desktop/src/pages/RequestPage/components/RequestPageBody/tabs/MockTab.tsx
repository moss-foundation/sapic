import { IDockviewPanelProps } from "@/lib/moss-tabs/src";
import { RequestPageProps } from "@/pages/RequestPage/RequestPage";

export const MockTab = ({ ...props }: IDockviewPanelProps<RequestPageProps>) => {
  return <div>MockTab</div>;
};
