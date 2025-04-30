// usePrepareWindow (in the root of the project):
import { useEffect, useState } from "react";

import { useDescribeAppState } from "./appState/useDescribeAppState";
import { useOpenWorkspace } from "./workspaces/useOpenWorkspace";

export interface WindowPreparationState {
  isPreparing: boolean;
}

export const usePrepareWindow = (): WindowPreparationState => {
  const [isPreparing, setIsPreparing] = useState(true);

  const { isFetched: isAppStateFetched, data: appState } = useDescribeAppState();
  const { mutate: openWorkspace, isSuccess: isOpenWorkspaceSuccess } = useOpenWorkspace();

  useEffect(() => {
    if (!isAppStateFetched) return;

    if (appState?.lastWorkspace && !isOpenWorkspaceSuccess) {
      openWorkspace(appState.lastWorkspace);
    }

    setIsPreparing(false);
  }, [isAppStateFetched, appState?.lastWorkspace, isOpenWorkspaceSuccess]);

  return { isPreparing };
};
