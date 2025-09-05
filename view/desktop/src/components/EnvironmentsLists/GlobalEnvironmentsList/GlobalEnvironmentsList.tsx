import { useStreamEnvironments } from "@/hooks";

import { EnvironmentListItem } from "../EnvironmentItem/EnvironmentListItem";

export const GlobalEnvironmentsList = () => {
  const { globalEnvironments } = useStreamEnvironments();

  return (
    <ul className="flex flex-col">
      {globalEnvironments?.map((environment) => (
        <EnvironmentListItem key={environment.id} environment={environment} type="global" />
      ))}
    </ul>
  );
};
