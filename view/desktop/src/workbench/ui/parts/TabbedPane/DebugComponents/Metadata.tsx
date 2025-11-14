import { DockviewPanelApi } from "moss-tabs";

import { Table, usePanelApiMetadata } from "./debugPanel";

const Metadata = ({ onClick, api }: { onClick: () => void; api: DockviewPanelApi }) => {
  const metadata = usePanelApiMetadata(api);

  return (
    <div className="text-sm">
      <Option title="Panel Rendering Mode" value={metadata.renderer.value} onClick={onClick} />

      <Table data={metadata} />
    </div>
  );
};

const Option = (props: { title: string; onClick: () => void; value: string }) => {
  return (
    <div>
      <span>{`${props.title}: `}</span>
      <button className="rounded !bg-cyan-300 p-2 !text-black hover:!bg-cyan-500" onClick={props.onClick}>
        {props.value}
      </button>
    </div>
  );
};

export default Metadata;
