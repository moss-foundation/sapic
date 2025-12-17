import { ParsedUrl } from "@/workbench/views/EndpointView/utils";

export interface ITokenizerIpc {
  //TODO: get ParsedUrl from the template-parser plugin
  getTokens: (url: string) => Promise<ParsedUrl>;
}
