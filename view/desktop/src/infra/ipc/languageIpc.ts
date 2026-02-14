import { ILanguageIpc } from "@/domains/language";

import { invokeTauriServiceIpc } from "./tauri";

export const languageIpc: ILanguageIpc = {
  listLanguages: async () => {
    return await invokeTauriServiceIpc("list_languages");
  },
  getTranslationNamespace: async (input) => {
    return await invokeTauriServiceIpc("get_translation_namespace", { input });
  },
};
