import { projectSummariesCollection } from "@/db/projectSummaries/projectSummaries";
import { projectIpc } from "@/infra/ipc/projectIpc";
import {
  BatchUpdateProjectInput,
  BatchUpdateProjectOutput,
  CreateProjectInput,
  CreateProjectOutput,
  DeleteProjectInput,
  DeleteProjectOutput,
  ImportProjectInput,
  ImportProjectOutput,
  ListProjectsOutput,
  UpdateProjectInput,
  UpdateProjectOutput,
} from "@repo/ipc";

import {
  batchUpdateProjectSummaryCollectionFromInput,
  updateProjectSummaryCollectionFromInput,
} from "./handlers/updateProjectSummaryDraftFromParams";

interface IProjectService {
  batchUpdateProject: (input: BatchUpdateProjectInput) => Promise<BatchUpdateProjectOutput>;

  createProject: (input: CreateProjectInput) => Promise<CreateProjectOutput>;

  deleteProject: (input: DeleteProjectInput) => Promise<DeleteProjectOutput>;

  importProject: (input: ImportProjectInput) => Promise<ImportProjectOutput>;

  listProjects: () => Promise<ListProjectsOutput>;

  updateProject: (input: UpdateProjectInput) => Promise<UpdateProjectOutput>;
}

export const projectService: IProjectService = {
  batchUpdateProject: async (input) => {
    const output = await projectIpc.batchUpdateProject(input);

    batchUpdateProjectSummaryCollectionFromInput(input);

    return output;
  },
  createProject: async (input) => {
    const output = await projectIpc.createProject(input);

    projectSummariesCollection.insert({
      id: output.id,
      name: input.name,
      archived: false,
      branch: null,
      iconPath: output.iconPath,

      expanded: true,
    });

    return output;
  },
  deleteProject: async (input) => {
    const output = await projectIpc.deleteProject(input);

    projectSummariesCollection.delete(input.id);

    return output;
  },
  importProject: async (input) => {
    return await projectIpc.importProject(input);
  },
  listProjects: async () => {
    return await projectIpc.listProjects();
  },
  updateProject: async (input) => {
    const output = await projectIpc.updateProject(input);

    updateProjectSummaryCollectionFromInput(input);

    return output;
  },
};
