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
    if (createParams) {
      if (gitProvider === null) {
        return true;
      }
    }
  }

  if (tab === "Import") {
    if (importParams) {
      if (gitProvider === null) {
        return true;
      }
    }
  }

  return false;
};
