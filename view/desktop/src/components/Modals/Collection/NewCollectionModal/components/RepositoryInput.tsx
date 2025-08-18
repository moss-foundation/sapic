import InputOutlined, { InputPlainProps } from "@/components/InputOutlined";

interface RepositoryInputProps extends InputPlainProps {
  repository: string;
  setRepository: (repository: string) => void;
}

export const RepositoryInput = ({ repository, setRepository, ...props }: RepositoryInputProps) => {
  return (
    <div className="col-span-2 grid grid-cols-subgrid items-center">
      <div>Repository:</div>
      <InputOutlined
        value={repository}
        className="max-w-72"
        onChange={(e) => setRepository(e.target.value)}
        placeholder="https://github.com/user/repo.git"
        required
        {...props}
      />
    </div>
  );
};
