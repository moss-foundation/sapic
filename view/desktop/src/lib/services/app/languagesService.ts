import { invokeTauriServiceIpc } from "@/lib/backend/tauri";
import { GetTranslationNamespaceInput, GetTranslationNamespaceOutput, ListLanguagesOutput } from "@repo/moss-app";

export const languagesService = {
  listLanguages: async () => {
    return await invokeTauriServiceIpc<void, ListLanguagesOutput>({ cmd: "list_languages" });
  },

  getTranslationNamespace: async (input: GetTranslationNamespaceInput) => {
    return await invokeTauriServiceIpc<GetTranslationNamespaceInput, GetTranslationNamespaceOutput>({
      cmd: "get_translation_namespace",
      args: {
        input,
      },
    });
  },
};
