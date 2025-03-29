import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export interface ViewGroup {
  id: string;
  title: string;
  order: number;
  icon: string;
}

export interface Views {
  viewGroups: ViewGroup[];
}

export interface GroupView {
  id: string;
  name: string;
  component: string;
}

export const USE_VIEW_GROUPS_QUERY_KEY = "viewGroups";
export const USE_CHANGE_VIEW_GROUPS_MUTATION_KEY = "changeViewGroups";
export const USE_VIEW_GROUP_QUERY_KEY = "viewGroup";

let Views: Views = {
  "viewGroups": [
    {
      "id": "collections.groupId",
      "title": "Collections",
      "order": 1,
      "icon": "ActivityBarCollectionsIcon",
    },
    {
      "id": "environments.groupId",
      "title": "Environments",
      "order": 2,
      "icon": "ActivityBarEnvironmentsIcon",
    },
    {
      "id": "mock.groupId",
      "title": "Mock Servers",
      "order": 3,
      "icon": "ActivityBarMockIcon",
    },
  ],
};

const getViewGroupsFn = async (): Promise<Views> => {
  await new Promise((resolve) => setTimeout(resolve, 50));
  return Views;
};

const changeViewGroupsFn = async (newViewGroups: Views): Promise<Views> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  Views = newViewGroups;

  return newViewGroups;
};

const getViewGroupFn = async (groupId: string): Promise<GroupView | null> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  if (groupId === "explorer.groupId") {
    return {
      "id": "explorer",
      "name": "My View1",
      "component": "AccordionsList",
    };
  }
  if (groupId === "activities.groupId") {
    return {
      "id": "activities",
      "name": "My View2",
      "component": "ActivitiesList",
    };
  }

  return null;
};

export const useGetViewGroups = () => {
  return useQuery<Views, Error>({
    queryKey: [USE_VIEW_GROUPS_QUERY_KEY],
    queryFn: getViewGroupsFn,
  });
};

export const useChangeViewGroups = () => {
  const queryClient = useQueryClient();

  return useMutation<Views, Error, Views>({
    mutationKey: [USE_CHANGE_VIEW_GROUPS_MUTATION_KEY],
    mutationFn: changeViewGroupsFn,
    onSuccess(newViewGroups) {
      queryClient.setQueryData([USE_VIEW_GROUPS_QUERY_KEY], newViewGroups);
    },
  });
};

export const useGetViewGroup = (groupId: string) => {
  return useQuery<GroupView | null, Error>({
    queryKey: [USE_VIEW_GROUP_QUERY_KEY, groupId],
    queryFn: () => getViewGroupFn(groupId),
    enabled: !!groupId,
  });
};
