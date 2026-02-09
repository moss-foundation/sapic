export type EnvironmentsDropOperations =
  | "ReorderWorkspaceEnvs"
  | "ReorderProjectEnvs"
  | "MoveWorkspaceEnvToProjectEnvs"
  | "MoveProjectEnvToWorkspaceEnvs"
  | "MoveProjectEnvToProjectEnv"
  | "CombineWorkspaceEnvToProjectList"
  | "CombineProjectEnvToProjectList"
  | null;
