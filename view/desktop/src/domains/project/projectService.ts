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
  listProjects: () => Promise<ListProjectsOutput>;
  importProject: (input: ImportProjectInput) => Promise<ImportProjectOutput>;

  createProject: (input: CreateProjectInput) => Promise<CreateProjectOutput>;

  updateProject: (input: UpdateProjectInput) => Promise<UpdateProjectOutput>;
  batchUpdateProject: (input: BatchUpdateProjectInput) => Promise<BatchUpdateProjectOutput>;

  deleteProject: (input: DeleteProjectInput) => Promise<DeleteProjectOutput>;
}

export const projectService: IProjectService = {
  listProjects: async () => {
    return await projectIpc.listProjects();
  },
  importProject: async (input) => {
    return await projectIpc.importProject(input);
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

  updateProject: async (input) => {
    const output = await projectIpc.updateProject(input);

    updateProjectSummaryCollectionFromInput(input);

    return output;
  },
  batchUpdateProject: async (input) => {
    const output = await projectIpc.batchUpdateProject(input);

    batchUpdateProjectSummaryCollectionFromInput(input);

    return output;
  },

  deleteProject: async (input) => {
    const output = await projectIpc.deleteProject(input);

    projectSummariesCollection.delete(input.id);

    return output;
  },
};
