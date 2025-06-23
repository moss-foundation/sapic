/**
 * TypeDoc plugin to add Git branch information to documentation
 */

const { execSync } = require("child_process");
const fs = require("fs");
const path = require("path");

function getGitInfo() {
  try {
    const branch = execSync("git rev-parse --abbrev-ref HEAD", { encoding: "utf8" }).trim();
    const commit = execSync("git rev-parse --short HEAD", { encoding: "utf8" }).trim();
    const timestamp = execSync("git log -1 --format=%cd --date=short", { encoding: "utf8" }).trim();
    return { branch, commit, timestamp };
  } catch (error) {
    console.warn("Warning: Could not determine Git information:", error.message);
    return { branch: "unknown", commit: "unknown", timestamp: "unknown" };
  }
}

function load(app) {
  let gitInfo = null;

  app.renderer.on("beginRender", (event) => {
    gitInfo = getGitInfo();

    event.project.gitBranch = gitInfo.branch;
    event.project.gitCommit = gitInfo.commit;
    event.project.gitTimestamp = gitInfo.timestamp;
    event.project.generatedAt = new Date().toISOString().split("T")[0];

    console.log(`📝 Documentation generated from branch: ${gitInfo.branch} (${gitInfo.commit})`);
    console.log(`📅 Last commit: ${gitInfo.timestamp}`);
  });

  app.renderer.on("endRender", (event) => {
    const outputDir = event.outputDirectory;

    if (!gitInfo) gitInfo = getGitInfo();

    const cssContent = `
.git-info-header {
  background: #1E2024;
  color: white;
  padding: 12px 20px;
  font-size: 13px;
  font-family: 'Menlo', 'Monaco', 'Consolas', monospace;
  border-bottom: 1px solid #4a5568;
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 10px;
}

.git-info-header .title {
  font-weight: medium;
  color: #e2e8f0;
  margin-right: 20px;
}

.git-info-header .info-group {
  display: flex;
  gap: 20px;
  align-items: center;
  flex-wrap: wrap;
}

.git-info-header .info-item {
  display: flex;
  align-items: center;
  gap: 5px;
}

.git-info-header .branch {
  font-weight: medium;
  color: #68d391;
}

.git-info-header .commit {
  color: #90cdf4;
  font-family: monospace;
}

.git-info-header .timestamp {
  color: #fbb6ce;
}

@media (max-width: 768px) {
  .git-info-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
  
  .git-info-header .info-group {
    gap: 15px;
  }
}
`;

    const generated = new Date().toISOString().split("T")[0];
    const jsContent = `
(function() {
  function addGitHeader() {
    const header = document.createElement('div');
    header.className = 'git-info-header';
    header.innerHTML = \`
      <div class="title">📋 Documentation Build Info</div>
      <div class="info-group">
        <div class="info-item">
          <span>Branch:</span>
          <span class="branch">${gitInfo.branch}</span>
        </div>
        <div class="info-item">
          <span>Commit:</span>
          <span class="commit">${gitInfo.commit}</span>
        </div>
        <div class="info-item">
          <span>Last commit:</span>
          <span class="timestamp">${gitInfo.timestamp}</span>
        </div>
        <div class="info-item">
          <span>Generated:</span>
          <span class="timestamp">${generated}</span>
        </div>
      </div>
    \`;
    
    const body = document.body;
    const firstScript = body.querySelector('script');
    if (firstScript) {
      body.insertBefore(header, firstScript.nextSibling);
    } else {
      body.insertBefore(header, body.firstChild);
    }
  }
  
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', addGitHeader);
  } else {
    addGitHeader();
  }
})();
`;

    const assetsDir = path.join(outputDir, "assets");
    fs.mkdirSync(assetsDir, { recursive: true });

    const cssPath = path.join(assetsDir, "git-info.css");
    const jsPath = path.join(assetsDir, "git-info.js");

    fs.writeFileSync(cssPath, cssContent);
    fs.writeFileSync(jsPath, jsContent);

    modifyHtmlFiles(outputDir);
  });
}

function modifyHtmlFiles(outputDir) {
  const htmlFiles = getAllHtmlFiles(outputDir);

  htmlFiles.forEach((htmlFile) => {
    let content = fs.readFileSync(htmlFile, "utf8");

    const relativePath = path.relative(path.dirname(htmlFile), outputDir);
    const assetsPath = relativePath ? `${relativePath}/assets` : "./assets";

    if (!content.includes("git-info.css") && content.includes("</head>")) {
      const cssLink = `  <link rel="stylesheet" href="${assetsPath}/git-info.css">\n`;
      content = content.replace("</head>", cssLink + "</head>");
    }

    if (!content.includes("git-info.js") && content.includes("</body>")) {
      const jsScript = `  <script src="${assetsPath}/git-info.js"></script>\n`;
      content = content.replace("</body>", jsScript + "</body>");
    }

    fs.writeFileSync(htmlFile, content);
  });
}

function getAllHtmlFiles(dir) {
  const files = [];

  function traverse(currentDir) {
    try {
      const items = fs.readdirSync(currentDir);

      items.forEach((item) => {
        const fullPath = path.join(currentDir, item);
        const stat = fs.statSync(fullPath);

        if (stat.isDirectory()) {
          traverse(fullPath);
        } else if (item.endsWith(".html")) {
          files.push(fullPath);
        }
      });
    } catch (error) {
      console.warn("Warning: Could not read directory:", currentDir, error.message);
    }
  }

  traverse(dir);
  return files;
}

module.exports = { load };
