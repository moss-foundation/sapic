import { AddAccountParams } from "@repo/moss-app";
import { CreateProjectGitParams, ImportProjectSource } from "@repo/moss-workspace";

import { CREATE_TAB, IMPORT_TAB } from "./constansts";

interface CalculateIsSubmitDisabledProps {
  name: string;
  tab: typeof CREATE_TAB | typeof IMPORT_TAB;
  createParams: CreateProjectGitParams | undefined;
  importParams: ImportProjectSource | undefined;
  gitProvider: AddAccountParams | null;
}

export const calculateIsSubmitDisabled = ({
  name,
  tab,
  createParams,
  importParams,
  gitProvider,
}: CalculateIsSubmitDisabledProps) => {
  if (!name) return true;

  if (tab === CREATE_TAB) {
    if (!createParams) {
      return false;
    }

    if ("gitHub" in createParams) {
      return !createParams.gitHub.repository || !createParams.gitHub.branch;
    }

    if ("gitLab" in createParams) {
      return !createParams.gitLab.repository || !createParams.gitLab.branch;
    }
  }

  if (tab === IMPORT_TAB) {
    if (!importParams) {
      return false;
    }

    if ("gitHub" in importParams) {
      return !importParams.gitHub.repository || !importParams.gitHub.branch || !importParams.gitHub.accountId;
    }

    if ("gitLab" in importParams) {
      return !importParams.gitLab.repository || !importParams.gitLab.branch || !importParams.gitLab.accountId;
    }
  }

  return false;
};
