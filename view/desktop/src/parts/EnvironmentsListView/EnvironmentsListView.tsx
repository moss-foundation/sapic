import { useEffect } from "react";

import { useStreamEnvironments } from "@/hooks/environment";

import { EnvironmentsListViewHeader } from "./EnvironmentsListViewHeader";

export const EnvironmentsListView = () => {
  const { data: environments } = useStreamEnvironments();

  useEffect(() => {
    console.log(environments);
  }, [environments]);

  return (
    <div>
      <EnvironmentsListViewHeader />

      <div className="p-4">
        <h3 className="text-lg font-semibold">Environments</h3>
        <p className="mt-2 text-sm text-gray-500">Environment management 21312312</p>
      </div>
    </div>
  );
};

export default EnvironmentsListView;
