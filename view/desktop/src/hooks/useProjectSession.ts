import {
  TREE_VIEW_GROUP_ENVIRONMENTS,
  TREE_VIEW_GROUP_MOCK_SERVERS,
  TREE_VIEW_GROUP_PROJECTS,
} from "@repo/moss-workspace";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

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
  "lastActiveGroup": TREE_VIEW_GROUP_PROJECTS,
  changedViews: [
    {
      "id": TREE_VIEW_GROUP_PROJECTS,
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
