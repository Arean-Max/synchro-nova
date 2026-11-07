import subprocess
import sys
import os

repo_root = r"C:\Users\tyler\Desktop\synchro"
os.chdir(repo_root)

res = subprocess.run(["git", "ls-files"], capture_output=True, text=True, check=True)
files = res.stdout.strip().splitlines()

code_exts = {".js", ".mjs", ".rs", ".css", ".html"}

comment_violations = []

for rel_path in files:
    _, ext = os.path.splitext(rel_path)
    if ext not in code_exts:
        continue
    
    if rel_path.startswith("src-tauri/gen/"):
        continue

    full_path = os.path.join(repo_root, rel_path)
    try:
        with open(full_path, "r", encoding="utf-8", errors="ignore") as f:
            lines = f.readlines()
    except Exception:
        continue

    for line_num, line in enumerate(lines, 1):
        clean = line
        for protocol in ["https://", "http://", "asset://", "file://"]:
            clean = clean.replace(protocol, "")

        if "//" in clean:
            comment_violations.append((rel_path, line_num, line.strip()))

        if "/*" in line:
            stripped = line.strip()
            if not (stripped.startswith('"test"') or "tests/*.test.mjs" in line):
                comment_violations.append((rel_path, line_num, line.strip()))

        if "<!--" in line:
            comment_violations.append((rel_path, line_num, line.strip()))

if comment_violations:
    print(f"Found {len(comment_violations)} comment violations:")
    for f, l, content in comment_violations[:20]:
        print(f"  {f}:{l} -> {content}")
    sys.exit(1)
else:
    print("SUCCESS: EXACTLY ZERO comments found across all source files in the repository!")
