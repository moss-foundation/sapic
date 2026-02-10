export enum ENVIRONMENT_ITEM_DRAG_TYPE {
  PROJECT = "ProjectEnvironmentItem",
  WORKSPACE = "WorkspaceEnvironmentItem",
}

export enum ENVIRONMENT_LIST_DRAG_TYPE {
  PROJECT = "ProjectEnvironmentList",
  WORKSPACE = "WorkspaceEnvironmentList",
}

export enum EnvironmentsDropOperations {
  ReorderWorkspaceEnvs = "ReorderWorkspaceEnvs",
  ReorderProjectEnvs = "ReorderProjectEnvs",

  MoveWorkspaceEnvToProjectEnvs = "MoveWorkspaceEnvToProjectEnvs",
  MoveProjectEnvToWorkspaceEnvs = "MoveProjectEnvToWorkspaceEnvs",
  MoveProjectEnvToProjectEnv = "MoveProjectEnvToProjectEnv",

  CombineWorkspaceEnvToProjectList = "CombineWorkspaceEnvToProjectList",
  CombineProjectEnvToProjectList = "CombineProjectEnvToProjectList",
  CombineProjectEnvToWorkspaceList = "CombineProjectEnvToWorkspaceList",
}
