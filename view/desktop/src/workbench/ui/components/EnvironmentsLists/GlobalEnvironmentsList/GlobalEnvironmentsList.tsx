import { useStreamEnvironments } from "@/adapters/tanstackQuery/environment";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "../constants";
import { EnvironmentListItem } from "../EnvironmentItem/EnvironmentListItem";

export const GlobalEnvironmentsList = () => {
  const { globalEnvironments } = useStreamEnvironments();

  return (
    <ul>
      {globalEnvironments?.map((environment) => (
        <EnvironmentListItem key={environment.id} environment={environment} type={ENVIRONMENT_ITEM_DRAG_TYPE.GLOBAL} />
      ))}
    </ul>
  );
};
