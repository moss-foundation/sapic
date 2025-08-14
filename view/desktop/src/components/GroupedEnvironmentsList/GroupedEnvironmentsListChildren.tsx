import { GroupedEnvironmentsListItem } from "./GroupedEnvironmentsListItem/GroupedEnvironmentsListItem";
import { GroupedWithEnvironment } from "./types";

interface GroupedEnvironmentsListChildrenProps {
  groupedWithEnvironments: GroupedWithEnvironment;
}

export const GroupedEnvironmentsListChildren = ({ groupedWithEnvironments }: GroupedEnvironmentsListChildrenProps) => {
  return (
    <div>
      {groupedWithEnvironments.environments.map((environment) => (
        <GroupedEnvironmentsListItem key={environment.id} environment={environment} />
      ))}
    </div>
  );
};
