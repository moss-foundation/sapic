import { AddAccountOutput } from "@repo/moss-app";
import { CreateCollectionGitParams, ImportCollectionSource } from "@repo/moss-workspace";

interface CalculateIsSubmitDisabledProps {
  name: string;
  tab: "Create" | "Import";
  createParams: CreateCollectionGitParams | undefined;
  importParams: ImportCollectionSource | undefined;
  gitProvider: AddAccountOutput | null;
}

export const calculateIsSubmitDisabled = ({
  name,
  tab,
  createParams,
  importParams,
  gitProvider,
}: CalculateIsSubmitDisabledProps) => {
  if (!name) return true;

  if (tab === "Create") {
    if (!createParams) {
      return true;
    }

    if ("gitHub" in createParams) {
      return !createParams.gitHub.repository || !createParams.gitHub.branch;
    }

    if ("gitLab" in createParams) {
      return !createParams.gitLab.repository || !createParams.gitLab.branch;
    }
  }

  if (tab === "Import") {
    if (!importParams) {
      return true;
    }

    if ("gitHub" in importParams) {
      return !importParams.gitHub.repository || !importParams.gitHub.branch;
    }

    if ("gitLab" in importParams) {
      return !importParams.gitLab.repository || !importParams.gitLab.branch;
    }
  }

  return false;
};
