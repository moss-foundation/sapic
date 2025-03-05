import { LocaleDescriptor } from "@repo/moss-nls";

import { invokeTauriIpc, IpcResult } from "./tauri";

const getLocales = async (): Promise<IpcResult<LocaleDescriptor[], string>> => {
  return await invokeTauriIpc<LocaleDescriptor[], string>("list_locales");
};

export default getLocales;
