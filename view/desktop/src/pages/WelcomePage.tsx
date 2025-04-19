import { Icon } from "@/components";
import Button from "@/components/Button";

export const WelcomePage = () => {
  return (
    <>
      <div className="relative flex h-full w-full flex-col gap-7.5 pt-32 pr-60 pl-36">
        <div className="flex justify-between">
          <FirstColumn />

          <div className="absolute inset-x-0 bottom-4 flex justify-center">
            <a
              className="flex animate-bounce cursor-pointer flex-col items-center gap-2 text-sm duration-1000"
              href="#TestAnchorForWelcomePage"
            >
              <span className="font-medium">Learn more</span>
              <Icon icon="ChevronDownEllipse" />
            </a>
          </div>

          <SecondColumn />
        </div>

        <StepsRow />
      </div>

      <div className="flex h-full w-full flex-col items-center pb-6">
        <a id="TestAnchorForWelcomePage" className="mt-auto">
          hello
        </a>
      </div>
    </>
  );
};

const StepsRow = () => {
  return (
    <div>
      <h3 className="text-xl font-medium">Next steps</h3>
      <div className="flex">
        <StepCard isNew />
        <StepCard />
        <StepCard />
        <StepCard isNew />
      </div>
    </div>
  );
};

const StepCard = ({ isNew = false }: { isNew?: boolean }) => {
  return (
    <div className="mx-4 my-3">
      <div className="flex items-center gap-1.5">
        <Icon icon="StepCardInfo" />
        <span className="font-medium">Learn the Fundamentals</span>
        {isNew && (
          <div className="background-(--moss-stepCard-bg) rounded-[3px] px-1 text-[11px] font-semibold text-(--moss-stepCard-text)">
            New
          </div>
        )}
      </div>
      <div className="text-(--moss-secondary-text)">
        Explain behavior that is not clear from the setting or action name.
      </div>
    </div>
  );
};

const FirstColumn = () => {
  return (
    <div className="flex flex-col gap-7.5">
      <div className="flex flex-col gap-4">
        <h1 className="text-[34px] font-medium">Simple API Client</h1>
        <p className="text-lg font-medium text-(--moss-secondary-text)">
          Design APIs, Send Requests, Unmatched Git Integration
        </p>
      </div>

      <div className="flex flex-col gap-2">
        <h2 className="text-lg font-medium">Start</h2>
        <button className="flex cursor-pointer gap-1.5">
          <Icon icon="FolderAdd" className="text-(--moss-primary)" />
          <span>New workspace</span>
        </button>
        <button className="flex cursor-pointer gap-1.5">
          <Icon icon="Folder" className="text-(--moss-primary)" />
          <span>Open workspace</span>
        </button>
      </div>

      <div className="flex flex-col gap-2">
        <h2 className="text-lg font-medium">Recent</h2>
        <div className="flex flex-col gap-1.5">
          <a href="#" className="text-(--moss-primary)">
            My Workspace
          </a>
          <a href="#" className="text-(--moss-primary)">
            Spaixel Monster
          </a>
          <a href="#" className="text-(--moss-primary)">
            Twinkle
          </a>
        </div>
      </div>
      <div>
        <Button variant="outlined" intent="neutral">
          More
        </Button>
      </div>
    </div>
  );
};

const SecondColumn = () => {
  return (
    <div className="flex max-w-[268px] flex-col gap-2">
      <h2 className="text-xl font-medium">Pin board</h2>
      <div>
        <p className="text-(--moss-secondary-text)">Lorem ipsum dolor sitel, consectetur adipiscing.</p>

        <WelcomePageDivider />

        <p className="text-(--moss-secondary-text)">
          Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
        </p>

        <WelcomePageDivider />

        <WelcomePageLink label="View Sapic’s Roadmap" />

        <WelcomePageDivider />

        <div className="flex flex-col gap-2">
          <h3 className="font-medium">Release pages:</h3>
          <div className="flex flex-col gap-2">
            <WelcomePageLink label="Quisque Faucibus" />
            <WelcomePageLink label="Tempus Leo" />
            <WelcomePageLink label="Lacinia Integer" />
          </div>
        </div>
      </div>
    </div>
  );
};

const WelcomePageLink = ({ label }: { label: string }) => {
  return (
    <div className="flex items-center text-(--moss-primary)">
      <a href="#">{label}</a>
      <Icon icon="ExternalLink" />
    </div>
  );
};

const WelcomePageDivider = () => {
  return <div className="background-(--moss-border-color) my-3 h-px w-full" />;
};

export default WelcomePage;
