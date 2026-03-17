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
  list: () => Promise<ListProjectsOutput>;
  import: (input: ImportProjectInput) => Promise<ImportProjectOutput>;

  create: (input: CreateProjectInput) => Promise<CreateProjectOutput>;

  update: (input: UpdateProjectInput) => Promise<UpdateProjectOutput>;
  batchUpdate: (input: BatchUpdateProjectInput) => Promise<BatchUpdateProjectOutput>;

  delete: (input: DeleteProjectInput) => Promise<DeleteProjectOutput>;
}

export const projectService: IProjectService = {
  list: async () => {
    const output = await projectIpc.list();

    output.items.forEach((project) => {
      if (projectSummariesCollection.has(project.id)) {
        projectSummariesCollection.update(project.id, (draft) => {
          draft.name = project.name;
          draft.archived = project.archived;
          draft.branch = project.branch;
          draft.iconPath = project.iconPath;
        });
      } else {
        projectSummariesCollection.insert({
          id: project.id,
          name: project.name,
          archived: project.archived,
          branch: project.branch,
          iconPath: project.iconPath,
          expanded: false,
        });
      }
    });

    return output;
  },
  import: async (input) => {
    const output = await projectIpc.import(input);

    projectSummariesCollection.insert({
      id: output.id,
      name: input.name,
      archived: false,
      branch: null,
      iconPath: output.iconPath,
      expanded: false,
    });

    return output;
  },

  create: async (input) => {
    const output = await projectIpc.create(input);

    projectSummariesCollection.insert({
      id: output.id,
      name: input.name,
      archived: false,
      branch: null,
      iconPath: output.iconPath,
      expanded: false,
    });

    return output;
  },

  update: async (input) => {
    const output = await projectIpc.update(input);

    updateProjectSummaryCollectionFromInput(input);

    return output;
  },
  batchUpdate: async (input) => {
    const output = await projectIpc.batchUpdate(input);

    batchUpdateProjectSummaryCollectionFromInput(input);

    return output;
  },

  delete: async (input) => {
    const output = await projectIpc.delete(input);

    projectSummariesCollection.delete(input.id);

    return output;
  },
};
