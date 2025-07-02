const fs = require("fs");
const path = require("path");

// Predefined categories configuration
const CATEGORY_CONFIG = {
  "Primitive": {
    title: "Primitives",
    color: "#8b5cf6",
    backgroundColor: "rgba(139, 92, 246, 0.05)",
    hoverBackgroundColor: "rgba(139, 92, 246, 0.1)",
  },
  "Type": {
    title: "Types",
    color: "#10b981",
    backgroundColor: "rgba(16, 185, 129, 0.05)",
    hoverBackgroundColor: "rgba(16, 185, 129, 0.1)",
  },
  "Operation": {
    title: "Operations",
    color: "#3b82f6",
    backgroundColor: "rgba(59, 130, 246, 0.05)",
    hoverBackgroundColor: "rgba(59, 130, 246, 0.1)",
  },
  "Event": {
    title: "Events",
    color: "#ef4444",
    backgroundColor: "rgba(239, 68, 68, 0.05)",
    hoverBackgroundColor: "rgba(239, 68, 68, 0.1)",
  },
};

function load(app) {
  console.log("Loading TypeDoc category plugin");

  // Listen for the end of rendering to modify HTML files
  app.renderer.on("endRender", () => {
    console.log("Adding category enhancements to documentation...");

    const outputDir = app.options.getValue("out") || "./autodocs";

    // Load categories from JSDoc tags
    const categories = loadCategoriesFromSource();
    console.log(`Found ${Object.keys(categories).length} categorized types`);

    // Process all HTML files
    processHtmlFiles(outputDir, categories);

    console.log("Category enhancement completed");
  });
}

function loadCategoriesFromSource() {
  const categories = {};
  const rootDir = path.resolve(__dirname, "../../");

  // Get entry points from typedoc.json
  const typedocPath = path.join(rootDir, "typedoc.json");
  let entryPoints = [];

  try {
    const config = JSON.parse(fs.readFileSync(typedocPath, "utf8"));
    entryPoints = config.entryPoints || [];
  } catch (error) {
    console.warn("Could not read typedoc.json:", error.message);
    return categories;
  }

  // Scan TypeScript files for @category tags
  entryPoints.forEach((entryPoint) => {
    const entryPath = path.join(rootDir, entryPoint);
    if (fs.existsSync(entryPath)) {
      scanDirectory(entryPath, categories);
    }
  });

  console.log(`Found ${Object.keys(categories).length} categorized types from JSDoc tags`);
  return categories;
}

function scanDirectory(dir, categories) {
  try {
    const items = fs.readdirSync(dir);

    items.forEach((item) => {
      const fullPath = path.join(dir, item);

      try {
        const stat = fs.statSync(fullPath);

        if (stat.isDirectory()) {
          if (!item.startsWith(".") && item !== "node_modules") {
            scanDirectory(fullPath, categories);
          }
        } else if (item.endsWith(".ts") && !item.endsWith(".d.ts")) {
          extractCategories(fullPath, categories);
        }
      } catch (statError) {
        // Silently skip files we can't stat (symlinks, permissions, etc.)
        // This is normal for some files like LICENSE-MIT symlinks
      }
    });
  } catch (error) {
    console.warn(`Error scanning ${dir}:`, error.message);
  }
}

function extractCategories(filePath, categories) {
  try {
    const content = fs.readFileSync(filePath, "utf8");
    const pattern = /\/\*\*[\s\S]*?@category\s+(\w+)[\s\S]*?\*\/\s*export\s+type\s+(\w+)/g;
    let match;
    let foundInFile = 0;

    while ((match = pattern.exec(content)) !== null) {
      const [, category, typeName] = match;
      // Only include categories that are defined in CATEGORY_CONFIG
      if (CATEGORY_CONFIG[category]) {
        categories[typeName] = category;
        foundInFile++;
      }
    }

    if (foundInFile > 0) {
      const relativePath = path.relative(path.resolve(__dirname, "../../"), filePath);
      console.log(`Found ${foundInFile} categories in ${relativePath}`);
    }
  } catch (error) {
    console.warn(`Error reading ${filePath}:`, error.message);
  }
}

function processHtmlFiles(outputDir, categories) {
  const htmlFiles = getAllHtmlFiles(outputDir);
  console.log(`Modifying ${htmlFiles.length} HTML files`);

  // CSS for category styling
  const categoryCSS = `
/* TypeDoc Category Enhancements */
.category-enhanced {
  padding-left: 8px;
  margin: 2px 0;
  transition: background-color 0.2s ease;
}

/* Remove border-left for root level items (modules) */
.tsd-navigation > ul > li.category-enhanced,
.tsd-navigation .tsd-small-nested-navigation > li.category-enhanced {
  border-left: none !important;
  padding-left: 0 !important;
  background-color: transparent !important;
}

/* Remove border-left for module level items */
nav.tsd-navigation > a.category-enhanced {
  border-left: none !important;
  background-color: transparent !important;
}

/* Category-specific styles - only for type items, not modules */
${Object.keys(CATEGORY_CONFIG)
  .map((categoryName) => {
    const config = CATEGORY_CONFIG[categoryName];
    const className = `category-${categoryName.toLowerCase()}`;

    return `
/* Sidebar navigation styles */
.tsd-navigation li:not(.tsd-navigation > ul > li):not(.tsd-small-nested-navigation > li).${className} {
  border-left: 4px solid ${config.color} !important;
  background-color: ${config.backgroundColor};
}

/* Hover state - limited width background for sidebar only */
.tsd-navigation li:not(.tsd-navigation > ul > li):not(.tsd-small-nested-navigation > li).${className}:hover::before {
  content: '';
  position: absolute;
  top: 0;
  left: -8px;
  right: -8px;
  bottom: 0;
  background-color: ${config.hoverBackgroundColor};
  z-index: -1;
}

.tsd-navigation li:not(.tsd-navigation > ul > li):not(.tsd-small-nested-navigation > li).${className}:hover {
  background-color: ${config.hoverBackgroundColor} !important;
  position: relative;
}

/* Active state styling for sidebar only */
.tsd-navigation li:not(.tsd-navigation > ul > li):not(.tsd-small-nested-navigation > li).${className}.current::before,
.tsd-navigation li:not(.tsd-navigation > ul > li):not(.tsd-small-nested-navigation > li).${className} a.current::before {
  content: '';
  position: absolute;
  top: 0;
  left: -8px;
  right: -8px;
  bottom: 0;
  background-color: ${config.hoverBackgroundColor};
  z-index: -1;
}

.tsd-navigation li:not(.tsd-navigation > ul > li):not(.tsd-small-nested-navigation > li).${className}.current,
.tsd-navigation li:not(.tsd-navigation > ul > li):not(.tsd-small-nested-navigation > li).${className} a.current {
  background-color: ${config.hoverBackgroundColor} !important;
  position: relative;
}

/* Category header squares applied via JavaScript */
.category-header-${config.title.toLowerCase()}::after {
  content: '';
  display: inline-block;
  width: 8px;
  height: 8px;
  background-color: ${config.color};
  margin-left: 8px;
  vertical-align: middle;
  border-radius: 1px;
}
`;
  })
  .join("\n")}

/* Default styles for non-categorized active items */
.tsd-navigation a.current:not(.category-enhanced),
.tsd-navigation a:focus:not(.category-enhanced),
.tsd-navigation a[aria-current="page"]:not(.category-enhanced) {
  background-color: rgba(128, 128, 128, 0.1) !important;
}

/* Category bar styles for left sidebar headers */
${Object.keys(CATEGORY_CONFIG)
  .map((categoryName) => {
    const config = CATEGORY_CONFIG[categoryName];
    const className = `category-bar-${config.title.toLowerCase()}`;

    return `
/* Vertical bar for category header in left sidebar */
.${className} {
  border-left: 4px solid ${config.color} !important;
  background-color: ${config.backgroundColor} !important;
  padding-left: 8px !important;
}

.${className}:hover {
  background-color: ${config.hoverBackgroundColor} !important;
}
`;
  })
  .join("\n")}

/* Ensure no icons/emojis are added */
.category-enhanced a::before {
  content: none !important;
}
`;

  // JavaScript to apply categories
  const categoryJS = `
(function() {
  const categories = ${JSON.stringify(categories)};
  
  // Helper function to normalize type names for matching
  function normalizeTypeName(name) {
    return name.replace(/[\\s\\-_<>]/g, '').toLowerCase();
  }
  
  // Create normalized lookup
  const normalizedCategories = {};
  Object.keys(categories).forEach(key => {
    normalizedCategories[normalizeTypeName(key)] = categories[key];
  });
  
  function enhanceNavigation() {
    let enhanced = 0;
    
    // Style sidebar navigation links (left column)
    const navLinks = document.querySelectorAll('.tsd-navigation a');
    navLinks.forEach(link => {
      const linkText = link.textContent?.trim();
      if (!linkText) return;
      
      // Skip if this is a top-level module link
      const isTopLevelModule = link.closest('.tsd-navigation > ul > li') || 
                              link.closest('.tsd-small-nested-navigation > li') ||
                              (link.parentElement && link.parentElement.tagName === 'NAV');
      
      if (isTopLevelModule && (linkText.startsWith('moss-') || linkText === 'Sapic')) {
        return; // Skip module-level items
      }
      
      const normalizedText = normalizeTypeName(linkText);
      const category = normalizedCategories[normalizedText];
      
      if (category) {
        const listItem = link.closest('li');
        if (listItem && !listItem.classList.contains('category-enhanced')) {
          listItem.classList.add('category-enhanced');
          listItem.classList.add('category-' + category.toLowerCase());
          enhanced++;
        }
      }
    });
    
    // Add colored squares to category headers in right sidebar
    const categoryTitles = ['Primitives', 'Types', 'Operations', 'Events'];
    categoryTitles.forEach(categoryTitle => {
      // Find elements that contain the category title as text
      const allElements = document.querySelectorAll('.tsd-page-navigation *');
      allElements.forEach(element => {
        if (element.textContent?.trim() === categoryTitle) {
          // Check if it's a category header (summary or h3)
          if (element.tagName === 'SUMMARY' || element.tagName === 'H3' || 
              element.classList.contains('tsd-accordion-summary')) {
            const className = 'category-header-' + categoryTitle.toLowerCase();
            if (!element.classList.contains(className)) {
              element.classList.add(className);
              enhanced++;
            }
          }
        }
      });
    });
    
    // Add vertical category bars to left sidebar items
    const categoryTitlesForBars = ['Primitives', 'Types', 'Operations', 'Events'];
    categoryTitlesForBars.forEach(categoryTitle => {
      // Find elements that are grouped under this category
      const allElements = document.querySelectorAll('.tsd-navigation a, .tsd-navigation summary');
      allElements.forEach(element => {
        // Check if this is a category header/summary
        if (element.textContent?.trim() === categoryTitle && 
            (element.tagName === 'SUMMARY' || element.classList.contains('tsd-accordion-summary'))) {
          const className = 'category-bar-' + categoryTitle.toLowerCase();
          if (!element.classList.contains(className)) {
            element.classList.add(className);
            enhanced++;
          }
        }
      });
    });
    
    if (enhanced > 0) {
      console.log('Enhanced', enhanced, 'navigation items with categories');
    }
    
    return enhanced;
  }
  
  // Run immediately
  enhanceNavigation();
  
  // Run when DOM is ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', enhanceNavigation);
  }
  
  // Use shorter intervals with smart stopping
  let attempts = 0;
  const maxAttempts = 10; // 10 * 200ms = 2 seconds
  
  const intervalId = setInterval(() => {
    attempts++;
    const enhanced = enhanceNavigation();
    
    // Stop if we enhanced something or reached max attempts
    if (enhanced > 0 || attempts >= maxAttempts) {
      clearInterval(intervalId);
    }
  }, 200);
  
  // Also run on page visibility change (when switching tabs)
  document.addEventListener('visibilitychange', function() {
    if (!document.hidden) {
      setTimeout(enhanceNavigation, 100);
    }
  });
})();
`;

  let processedCount = 0;

  htmlFiles.forEach((htmlFile) => {
    try {
      let content = fs.readFileSync(htmlFile, "utf8");
      let modified = false;

      // Add CSS if not already present
      if (!content.includes("typedoc-categories-css")) {
        content = content.replace("</head>", `<style id="typedoc-categories-css">\n${categoryCSS}\n</style>\n</head>`);
        modified = true;
      }

      // Add JavaScript if not already present
      if (!content.includes("typedoc-categories-js")) {
        content = content.replace("</body>", `<script id="typedoc-categories-js">\n${categoryJS}\n</script>\n</body>`);
        modified = true;
      }

      if (modified) {
        fs.writeFileSync(htmlFile, content);
        processedCount++;
      }
    } catch (error) {
      console.warn(`Error processing ${htmlFile}:`, error.message);
    }
  });

  console.log(`Enhanced ${processedCount} HTML files`);
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
      console.warn(`Error reading ${currentDir}:`, error.message);
    }
  }

  traverse(dir);
  return files;
}

module.exports = { load };
