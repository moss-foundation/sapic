import { languageIpc } from "@/infra/ipc/languageIpc";
import { LanguageInfo } from "@repo/base";
import { GetTranslationNamespaceInput, GetTranslationNamespaceOutput } from "@repo/ipc";

interface ILanguageService {
  listLanguages: () => Promise<LanguageInfo[]>;
  getTranslationNamespace: (input: GetTranslationNamespaceInput) => Promise<GetTranslationNamespaceOutput>;
}

export const languageService: ILanguageService = {
  listLanguages: async () => {
    return await languageIpc.listLanguages();
  },
  getTranslationNamespace: async (input) => {
    return await languageIpc.getTranslationNamespace(input);
  },
};
