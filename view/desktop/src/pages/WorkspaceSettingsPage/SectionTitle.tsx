interface SectionTitleProps {
  children: React.ReactNode;
  className?: string;
}

export const SectionTitle = ({ children, className = "" }: SectionTitleProps) => {
  return (
    <div className="mb-4 flex items-center gap-4">
      <h3 className={`font-medium ${className}`}>{children}</h3>
      <hr className="max-w-96 flex-1 border-gray-300" />
    </div>
  );
};
