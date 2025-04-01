import { ActivityBar } from "./ActivityBar";

interface HorizontalActivityBarProps {
  position: "top" | "bottom";
}

export const HorizontalActivityBar = ({ position }: HorizontalActivityBarProps) => {
  return (
    <div className="w-full flex-shrink-0">
      <ActivityBar />
    </div>
  );
};

export default HorizontalActivityBar;
