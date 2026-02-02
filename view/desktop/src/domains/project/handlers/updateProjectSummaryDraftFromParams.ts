import { projectSummariesCollection } from "@/db/projectSummaries/projectSummaries";
import { ProjectSummary } from "@/db/projectSummaries/types";
import { BatchUpdateProjectInput, UpdateProjectInput, UpdateProjectParams } from "@repo/ipc";

const updateProjectSummaryDraftFromParams = (draft: ProjectSummary, params: UpdateProjectParams) => {
  if (params.name !== undefined) {
    draft.name = params.name;
  }
  if (params.order !== undefined) {
    draft.order = params.order;
  }
  if (params.expanded !== undefined) {
    draft.expanded = params.expanded;
  }
  if (params.iconPath !== undefined) {
    if (params.iconPath === "REMOVE") {
      draft.iconPath = null;
    } else if (typeof params.iconPath === "object" && "UPDATE" in params.iconPath) {
      draft.iconPath = params.iconPath.UPDATE;
    }
  }
};

export const updateProjectSummaryCollectionFromInput = (input: UpdateProjectInput) => {
  projectSummariesCollection.update(input.id, (draft) => updateProjectSummaryDraftFromParams(draft, input));
};

export const batchUpdateProjectSummaryCollectionFromInput = (input: BatchUpdateProjectInput) => {
  input.items.forEach((item) => {
    projectSummariesCollection.update(item.id, (draft) => updateProjectSummaryDraftFromParams(draft, item));
  });
};
