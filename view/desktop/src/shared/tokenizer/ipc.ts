import { ParsedUrl } from "@/workbench/views/EndpointView/utils";

export interface ITokenizerIpc {
  //TODO: get ParsedUrl from the url-parser plugin
  getTokens: (url: string) => Promise<ParsedUrl>;
}
