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
  StreamProjectsEvent,
  UpdateProjectInput,
  UpdateProjectOutput,
} from "@repo/ipc";
import { Channel } from "@tauri-apps/api/core";
import {
  batchUpdateProjectSummaryCollectionFromInput,
  updateProjectSummaryCollectionFromInput,
} from "./handlers/updateProjectSummaryDraftFromParams";

interface IProjectService {
  batchUpdateProject: (input: BatchUpdateProjectInput) => Promise<BatchUpdateProjectOutput>;

  createProject: (input: CreateProjectInput) => Promise<CreateProjectOutput>;

  deleteProject: (input: DeleteProjectInput) => Promise<DeleteProjectOutput>;

  importProject: (input: ImportProjectInput) => Promise<ImportProjectOutput>;

  streamProjects: (channelEvent: Channel<StreamProjectsEvent>) => Promise<StreamProjectsEvent[]>;

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

      order: input.order,
      expanded: output.expanded,
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
  streamProjects: async (channelEvent) => {
    return await projectIpc.streamProjects(channelEvent);
  },
  updateProject: async (input) => {
    const output = await projectIpc.updateProject(input);

    updateProjectSummaryCollectionFromInput(input);

    return output;
  },
};
