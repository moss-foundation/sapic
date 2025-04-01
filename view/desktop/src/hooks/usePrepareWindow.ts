import { useEffect, useState } from "react";

export interface WindowPreparationState {
  isPreparing: boolean;
}

const prepareWindowFn = (): WindowPreparationState => {
  return { isPreparing: false };
};

export const usePrepareWindow = (): WindowPreparationState => {
  const [isPreparing, setIsPreparing] = useState(true);

  useEffect(() => {
    setIsPreparing(false);
  }, []);

  return { isPreparing };
};
