import { ILanguageIpc } from "@/domains/language";

import { invokeTauriIpc } from "./tauri";

export const languageIpc: ILanguageIpc = {
  listLanguages: async () => {
    return await invokeTauriIpc("list_languages");
  },
  getTranslationNamespace: async (input) => {
    return await invokeTauriIpc("get_translation_namespace", { input });
  },
};
