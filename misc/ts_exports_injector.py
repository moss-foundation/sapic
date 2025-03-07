import os
from pathlib import Path

if __name__ == "__main__":
    with open("index.ts", mode="w") as f:
        for root, dirs, files in os.walk("bindings"):
            for file in files:
                import_path = "./" + root.replace("\\", "/") + "/" + Path(file).stem
                f.write(f"export * from \"{import_path}\";\n")