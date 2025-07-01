import json
import re
import subprocess
import os

pattern = r"crates/moss-(\w+)/src/models*"

if __name__ == "__main__":
    updated_models = set()
    base = os.environ.get("GITHUB_BASE_REF", "main")
    subprocess.run(["git", "fetch", "origin", f"{base}:{base}"], check=True, text=True, capture_output=True)

    diff_output = subprocess.run(["git", "diff", f"origin/{base}...HEAD", "--name-only"], check=True, text=True,
                                 capture_output=True)
    
    changed_files = diff_output.stdout.splitlines() if diff_output else []
    for changed_file in changed_files:
        match = re.search(pattern, changed_file)
        if match:
            updated_models.add(match.group(1))

    json_output = json.dumps(list(updated_models))
    with open(os.environ['GITHUB_OUTPUT'], 'a') as fh:
        fh.write(f'UPDATED_MODELS={json_output}\n')
