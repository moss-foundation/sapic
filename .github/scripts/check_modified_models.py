import json
import re
import subprocess
import os

pattern = r"crates/moss-(\w+)/src/models*"

if __name__ == "__main__":
    updated_models = set()

    command = ["git", "diff", "origin...HEAD", "--name-only"]
    result = subprocess.run(command, capture_output=True, text=True, check=True)
    changed_files = result.stdout.strip().split('\n')
    for changed_file in changed_files:
        match = re.search(pattern, changed_file)
        if match:
            updated_models.add(match.group(1))

    json_output = json.dumps(list(updated_models))
    with open(os.environ['GITHUB_OUTPUT'], 'a') as fh:
        fh.write(f'UPDATED_MODELS={json_output}\n')
