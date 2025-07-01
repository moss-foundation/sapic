/**
 * TypeDoc plugin to categorize entities by Operations, Types, Events, and Primitives
 * Creates collapsible groups in the documentation navigation
 */

function load(app) {
  // Categories mapping with their respective icons and descriptions
  const categories = {
    Operations: {
      icon: "⚙️",
      description: "API Operations and Commands",
      patterns: ["Input", "Output", "Command", "Request", "Response"],
      filePatterns: ["operations"],
      color: "#4F46E5", // Indigo
    },
    Types: {
      icon: "📋",
      description: "Data Types and Structures",
      patterns: ["Info", "Config", "Settings", "State", "Data"],
      filePatterns: ["types"],
      color: "#059669", // Emerald
    },
    Events: {
      icon: "📢",
      description: "Events and Event Payloads",
      patterns: ["Event", "Payload", "Handler", "Listener"],
      filePatterns: ["events"],
      color: "#DC2626", // Red
    },
    Primitives: {
      icon: "🧱",
      description: "Primitive Types and Enums",
      patterns: ["Id", "Level", "Mode", "Status"],
      filePatterns: ["primitives"],
      color: "#7C2D12", // Amber
    },
  };

  // Function to determine category based on reflection name and source
  function categorizeReflection(reflection) {
    if (!reflection.name) return null;

    // Check by source file name first (most reliable)
    if (reflection.sources && reflection.sources.length > 0) {
      const sourcePath = reflection.sources[0].fileName.toLowerCase();

      for (const [categoryName, category] of Object.entries(categories)) {
        if (category.filePatterns.some((pattern) => sourcePath.includes(pattern))) {
          return categoryName;
        }
      }
    }

    // Fallback to name pattern matching
    const reflectionName = reflection.name;

    for (const [categoryName, category] of Object.entries(categories)) {
      if (category.patterns.some((pattern) => reflectionName.includes(pattern))) {
        return categoryName;
      }
    }

    return null;
  }

  // Add custom CSS and JavaScript for collapsible groups
  app.renderer.on("endRender", (event) => {
    const outputDir = event.outputDirectory;
    const fs = require("fs");
    const path = require("path");

    // Custom CSS for categorized navigation
    const categoryCSS = `
/* Category Styles */
.tsd-navigation .category-group {
  margin: 8px 0;
  border: 1px solid var(--color-accent);
  border-radius: 6px;
  overflow: hidden;
  background: var(--color-background-secondary);
}

.tsd-navigation .category-header {
  padding: 8px 12px;
  background: var(--color-background);
  border-bottom: 1px solid var(--color-accent);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-weight: 600;
  font-size: 0.9em;
  transition: background-color 0.2s ease;
  user-select: none;
}

.tsd-navigation .category-header:hover {
  background: var(--color-accent);
}

.tsd-navigation .category-content {
  max-height: 1000px;
  overflow: hidden;
  transition: max-height 0.3s ease;
}

.tsd-navigation .category-content.collapsed {
  max-height: 0;
}

.tsd-navigation .category-toggle {
  transition: transform 0.2s ease;
  font-size: 0.8em;
}

.tsd-navigation .category-toggle.collapsed {
  transform: rotate(-90deg);
}

.tsd-navigation .category-item {
  padding-left: 20px;
}

/* Color coding for different categories */
.category-Operations .category-header { 
  border-left: 4px solid #4F46E5; 
}
.category-Types .category-header { 
  border-left: 4px solid #059669; 
}
.category-Events .category-header { 
  border-left: 4px solid #DC2626; 
}
.category-Primitives .category-header { 
  border-left: 4px solid #7C2D12; 
}

/* Responsive design */
@media (max-width: 768px) {
  .tsd-navigation .category-header {
    padding: 6px 10px;
    font-size: 0.8em;
  }
  
  .tsd-navigation .category-item {
    padding-left: 15px;
  }
}

/* Category grouping in navigation */
.tsd-navigation .category-group-operations {
  border-left: 3px solid #4F46E5;
  background: rgba(79, 70, 229, 0.05);
}

.tsd-navigation .category-group-types {
  border-left: 3px solid #059669;
  background: rgba(5, 150, 105, 0.05);
}

.tsd-navigation .category-group-events {
  border-left: 3px solid #DC2626;
  background: rgba(220, 38, 38, 0.05);
}

.tsd-navigation .category-group-primitives {
  border-left: 3px solid #7C2D12;
  background: rgba(124, 45, 18, 0.05);
}
`;

    // JavaScript for collapsible functionality and navigation enhancement
    const categoryJS = `
(function() {
  const categories = {
    Operations: { icon: "⚙️", patterns: ["Input", "Output", "Command", "Request", "Response"], filePatterns: ["operations"] },
    Types: { icon: "📋", patterns: ["Info", "Config", "Settings", "State", "Data"], filePatterns: ["types"] },
    Events: { icon: "📢", patterns: ["Event", "Payload", "Handler", "Listener"], filePatterns: ["events"] },
    Primitives: { icon: "🧱", patterns: ["Id", "Level", "Mode", "Status"], filePatterns: ["primitives"] }
  };

  function categorizeElement(element) {
    const text = element.textContent || '';
    const href = element.getAttribute('href') || '';
    
    // Check by file patterns first
    for (const [categoryName, category] of Object.entries(categories)) {
      if (category.filePatterns.some(pattern => href.toLowerCase().includes(pattern))) {
        return categoryName;
      }
    }
    
    // Fallback to name patterns
    for (const [categoryName, category] of Object.entries(categories)) {
      if (category.patterns.some(pattern => text.includes(pattern))) {
        return categoryName;
      }
    }
    
    return null;
  }

  function initializeCategoryCollapse() {
    // Find all category headers
    const categoryHeaders = document.querySelectorAll('.category-header');
    
    categoryHeaders.forEach(header => {
      header.addEventListener('click', function(e) {
        e.preventDefault();
        e.stopPropagation();
        
        const content = this.nextElementSibling;
        const toggle = this.querySelector('.category-toggle');
        
        if (content && content.classList.contains('category-content')) {
          content.classList.toggle('collapsed');
          if (toggle) toggle.classList.toggle('collapsed');
          
          // Save state to localStorage
          const categoryName = this.textContent.trim();
          const isCollapsed = content.classList.contains('collapsed');
          localStorage.setItem('typedoc-category-' + categoryName, isCollapsed.toString());
        }
      });
      
      // Restore state from localStorage
      const categoryName = header.textContent.trim();
      const savedState = localStorage.getItem('typedoc-category-' + categoryName);
      
      if (savedState === 'true') {
        const content = header.nextElementSibling;
        const toggle = header.querySelector('.category-toggle');
        if (content) {
          content.classList.add('collapsed');
          if (toggle) toggle.classList.add('collapsed');
        }
      }
    });
  }

  function enhanceNavigation() {
    // Find navigation links and group them by category
    const navLinks = document.querySelectorAll('.tsd-navigation a');
    const categoryGroups = {};
    
    navLinks.forEach(link => {
      const category = categorizeElement(link);
      if (category) {
        if (!categoryGroups[category]) {
          categoryGroups[category] = [];
        }
        categoryGroups[category].push(link.parentElement);
        
        // Add category class to the parent
        const listItem = link.closest('li');
        if (listItem) {
          listItem.classList.add('category-group-' + category.toLowerCase());
        }
      }
    });

    // Add visual indicators
    Object.entries(categoryGroups).forEach(([categoryName, elements]) => {
      const categoryInfo = categories[categoryName];
      elements.forEach(element => {
        const link = element.querySelector('a');
        if (link && !link.textContent.startsWith(categoryInfo.icon)) {
          // Add icon prefix to the link text
          const icon = document.createElement('span');
          icon.textContent = categoryInfo.icon + ' ';
          icon.style.marginRight = '4px';
          link.insertBefore(icon, link.firstChild);
        }
      });
    });
  }
  
  // Initialize when DOM is ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', function() {
      initializeCategoryCollapse();
      enhanceNavigation();
    });
  } else {
    initializeCategoryCollapse();
    enhanceNavigation();
  }
})();
`;

    // Write CSS and JS files
    const assetsDir = path.join(outputDir, "assets");
    if (!fs.existsSync(assetsDir)) {
      fs.mkdirSync(assetsDir, { recursive: true });
    }

    fs.writeFileSync(path.join(assetsDir, "category-styles.css"), categoryCSS);

    fs.writeFileSync(path.join(assetsDir, "category-behavior.js"), categoryJS);

    // Inject into HTML files
    modifyHtmlFiles(outputDir);

    console.log("📊 Category enhancement added to TypeDoc documentation");
  });
}

function modifyHtmlFiles(outputDir) {
  const fs = require("fs");
  const path = require("path");

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

  const htmlFiles = getAllHtmlFiles(outputDir);

  htmlFiles.forEach((htmlFile) => {
    let content = fs.readFileSync(htmlFile, "utf8");

    const relativePath = path.relative(path.dirname(htmlFile), outputDir);
    const assetsPath = relativePath ? `${relativePath}/assets` : "./assets";

    // Add CSS
    if (!content.includes("category-styles.css") && content.includes("</head>")) {
      const cssLink = `  <link rel="stylesheet" href="${assetsPath}/category-styles.css">\n`;
      content = content.replace("</head>", cssLink + "</head>");
    }

    // Add JavaScript
    if (!content.includes("category-behavior.js") && content.includes("</body>")) {
      const jsScript = `  <script src="${assetsPath}/category-behavior.js"></script>\n`;
      content = content.replace("</body>", jsScript + "</body>");
    }

    fs.writeFileSync(htmlFile, content);
  });
}

module.exports = { load };
