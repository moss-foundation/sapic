import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import {
  TREE_VIEW_GROUP_COLLECTIONS,
  TREE_VIEW_GROUP_ENVIRONMENTS,
  TREE_VIEW_GROUP_MOCK_SERVERS,
} from "@repo/moss-workspace";

export interface ProjectSessionState {
  lastActiveGroup: string;
  changedViews: ChangedView[];
}

interface ChangedView {
  id: string;
  collapsed: boolean;
}

export const USE_PROJECT_SESSION_STATE_QUERY_KEY = "projectSessionState";
export const USE_CHANGE_PROJECT_SESSION_STATE_MUTATION_KEY = "changeProjectSessionState";

// FIXME: remove mock data
let projectSessionState = {
  "lastActiveGroup": TREE_VIEW_GROUP_COLLECTIONS,
  changedViews: [
    {
      "id": TREE_VIEW_GROUP_COLLECTIONS,
      collapsed: false,
    },
    {
      "id": TREE_VIEW_GROUP_ENVIRONMENTS,
      collapsed: false,
    },
    {
      "id": TREE_VIEW_GROUP_MOCK_SERVERS,
      collapsed: false,
    },
  ],
};

const getProjectSessionStateFn = async (): Promise<ProjectSessionState> => {
  await new Promise((resolve) => setTimeout(resolve, 50));
  return projectSessionState;
};

const changeProjectSessionStateFn = async (newProjectState: ProjectSessionState): Promise<ProjectSessionState> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  projectSessionState = newProjectState;

  return newProjectState;
};

export const useGetProjectSessionState = () => {
  return useQuery<ProjectSessionState, Error>({
    queryKey: [USE_PROJECT_SESSION_STATE_QUERY_KEY],
    queryFn: getProjectSessionStateFn,
  });
};

export const useChangeProjectSessionState = () => {
  const queryClient = useQueryClient();

  return useMutation<ProjectSessionState, Error, ProjectSessionState>({
    mutationKey: [USE_CHANGE_PROJECT_SESSION_STATE_MUTATION_KEY],
    mutationFn: changeProjectSessionStateFn,
    onSuccess(newProjectState) {
      queryClient.setQueryData([USE_PROJECT_SESSION_STATE_QUERY_KEY], newProjectState);
    },
  });
};
