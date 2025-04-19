import { Icon } from "@/components";

export const WelcomePage = () => {
  return (
    <>
      <div className="relative flex h-full w-full justify-between pt-32 pr-60 pl-36">
        <div className="flex flex-col gap-7.5">
          <div className="flex flex-col gap-4">
            <h1 className="text-[34px]">Simple API Client</h1>
            <p className="text-lg">Design APIs, Send Requests, Unmatched Git Integration</p>
          </div>

          <div className="flex flex-col gap-2">
            <h2>Start</h2>
            <div className="flex gap-1.5">
              <Icon icon="FolderAdd" className="text-(--moss-primary)" />
              <span>New workspace</span>
            </div>
            <div className="flex gap-1.5">
              <Icon icon="Folder" className="text-(--moss-primary)" />
              <span>Open workspace</span>
            </div>
          </div>

          <div className="flex flex-col gap-2">
            <h2>Recent</h2>
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
        </div>

        <div className="absolute inset-x-0 bottom-4 flex justify-center">
          <a
            className="flex animate-bounce cursor-pointer flex-col items-center gap-2 duration-1000"
            href="#TestAnchorForWelcomePage"
          >
            <span className="font-medium">Learn more</span>
            <Icon icon="ChevronDownEllipse" />
          </a>
        </div>

        <div></div>
      </div>

      <div className="flex h-full min-h-screen w-full flex-col items-center justify-end pb-6">
        <a id="TestAnchorForWelcomePage" className="mt-auto">
          hello
        </a>
      </div>
    </>
  );
};

export default WelcomePage;
