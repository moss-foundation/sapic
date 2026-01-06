import { IDockviewPanelProps } from "moss-tabs";

import { Icons } from "@/lib/ui";

export interface DefaultViewProps<TParams extends Record<string, unknown> = Record<string, unknown>>
  extends Omit<IDockviewPanelProps<TParams & { [key: string]: unknown; tabIcon?: Icons }>, "params"> {
  params: TParams & { [key: string]: unknown; tabIcon?: Icons };
}
