interface SectionTitleProps {
  children: React.ReactNode;
  className?: string;
}

export const SectionTitle = ({ children, className = "" }: SectionTitleProps) => {
  return (
    <div className="mb-2.5 flex w-full items-center gap-4">
      <h3 className={`font-medium ${className}`}>{children}</h3>
      <hr className="flex-1 border-(--moss-border-color)" />
    </div>
  );
};
