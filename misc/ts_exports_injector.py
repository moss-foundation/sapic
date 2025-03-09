import os
from pathlib import Path

if __name__ == "__main__":
    output = []
    with open("index.ts", mode="w") as f:
        for root, dirs, files in os.walk("bindings"):
            for file in files:
                import_path = "./" + root.replace("\\", "/") + "/" + Path(file).stem
                output.append(f"export * from \"{import_path}\";")
        f.write("\n".join(output))
