import { LanguageInfo } from "@repo/base";
import { GetTranslationNamespaceInput, GetTranslationNamespaceOutput } from "@repo/ipc";

export interface ILanguageIpc {
  listLanguages: () => Promise<LanguageInfo[]>;

  getTranslationNamespace: (input: GetTranslationNamespaceInput) => Promise<GetTranslationNamespaceOutput>;
}
