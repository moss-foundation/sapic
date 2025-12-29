import { ILanguageIpc } from "@/domains/language";
import { invoke } from "@tauri-apps/api/core";

export const languageIpc: ILanguageIpc = {
  listLanguages: async () => {
    return await invoke("list_languages");
  },
  getTranslationNamespace: async (input) => {
    return await invoke("get_translation_namespace", { input });
  },
};
