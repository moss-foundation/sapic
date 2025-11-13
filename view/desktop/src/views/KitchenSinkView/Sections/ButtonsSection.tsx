import { useState } from "react";

import { Button } from "@/lib/ui";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const ButtonsSection = () => {
  const [button1Loading, setButton1Loading] = useState(false);
  const [button2Loading, setButton2Loading] = useState(false);
  const [button3Loading, setButton3Loading] = useState(false);
  const [button4Loading, setButton4Loading] = useState(false);

  const handleButton1Click = () => {
    setButton1Loading(true);
    setTimeout(() => setButton1Loading(false), 3000);
  };

  const handleButton2Click = () => {
    setButton2Loading(true);
    setTimeout(() => setButton2Loading(false), 3000);
  };

  const handleButton3Click = () => {
    setButton3Loading(true);
    setTimeout(() => setButton3Loading(false), 3000);
  };

  const handleButton4Click = () => {
    setButton4Loading(true);
    setTimeout(() => setButton4Loading(false), 3000);
  };
  return (
    <KitchenSinkSection
      header="Button Components"
      description="Various button states and variants available in the application."
    >
      <div className="flex flex-col gap-2 overflow-hidden">
        <div className="grid grid-cols-[150px_min-content] justify-items-start gap-2">
          <Button intent="primary">ButtonPrimary</Button>
          <Button intent="primary" disabled>
            ButtonPrimaryDisabled
          </Button>

          <Button intent="outlined">ButtonNeutralOutlined</Button>
          <Button intent="outlined" disabled>
            ButtonNeutralOutlinedDisabled
          </Button>

          <Button intent="danger">ButtonDanger</Button>
          <Button intent="danger" disabled>
            ButtonDangerDisabled
          </Button>
        </div>

        <Button fullWidth>Button Full Width</Button>

        <div>
          <div className="flex gap-2">
            <Button loading={button1Loading} onClick={handleButton1Click} intent="primary">
              Click to load
            </Button>
            <Button loading={button2Loading} onClick={handleButton2Click} intent="outlined">
              Click to load
            </Button>
            <Button loading={button3Loading} onClick={handleButton3Click} intent="danger">
              Click to load
            </Button>
            <Button loading={button4Loading} onClick={handleButton4Click} iconLeft="Copy">
              Click to load
            </Button>

            <Button loading={button4Loading} onClick={handleButton4Click} iconRight="Copy">
              Click to load
            </Button>

            <Button loading={button4Loading} onClick={handleButton4Click} iconLeft="Copy" iconRight="Copy">
              Click to load
            </Button>

            <Button loading={button4Loading} onClick={handleButton4Click} icon="Copy">
              Click to load
            </Button>
          </div>
        </div>
      </div>
    </KitchenSinkSection>
  );
};
