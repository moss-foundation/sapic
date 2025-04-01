import { ActivityBar } from "./ActivityBar";

interface VerticalActivityBarProps {
  position: "left" | "right";
}

export const VerticalActivityBar = ({ position }: VerticalActivityBarProps) => {
  return (
    <div
      className="flex h-full flex-shrink-0"
      style={{
        position: "absolute",
        top: 0,
        bottom: 0,
        [position]: 0,
        width: "41px",
        zIndex: 5,
        boxShadow: position === "left" ? "1px 0 3px rgba(0,0,0,0.1)" : "-1px 0 3px rgba(0,0,0,0.1)",
        background: "var(--moss-activityBar-background)",
        borderRight: position === "left" ? "1px solid var(--moss-activityBar-border-color)" : "none",
        borderLeft: position === "right" ? "1px solid var(--moss-activityBar-border-color)" : "none",
      }}
    >
      <ActivityBar />
    </div>
  );
};

export default VerticalActivityBar;
