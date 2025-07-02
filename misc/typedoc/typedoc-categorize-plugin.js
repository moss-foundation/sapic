/**
 * TypeDoc plugin to categorize entities by Operations, Types, Events, and Primitives
 * Creates visual grouping with icons and color coding in the documentation navigation
 */

const fs = require("fs");
const path = require("path");

function load(app) {
  console.log("Loading category enhancement plugin");

  // Preload JSDoc categories from TypeScript files
  const typeCategories = preloadTypeCategories();
  console.log("Preloaded categories for", Object.keys(typeCategories).length, "types");

  // Brief summary of categories
  const categorySummary = {};
  Object.values(typeCategories).forEach((category) => {
    categorySummary[category] = (categorySummary[category] || 0) + 1;
  });

  console.log(
    "Category distribution:",
    Object.entries(categorySummary)
      .map(([cat, count]) => `${cat}(${count})`)
      .join(", ")
  );

  const categories = {
    Operations: {
      icon: "⚙️",
    },
    Types: {
      icon: "📋",
    },
    Events: {
      icon: "📢",
    },
    Primitives: {
      icon: "🧱",
    },
  };

  // Add custom CSS and JavaScript after rendering
  app.renderer.on("endRender", (event) => {
    try {
      const outputDir = event.outputDirectory;
      console.log("Enhancing documentation with categories at:", outputDir);

      // Create assets directory if it doesn't exist
      const assetsDir = path.join(outputDir, "assets");
      if (!fs.existsSync(assetsDir)) {
        fs.mkdirSync(assetsDir, { recursive: true });
      }

      // Custom CSS for categorized navigation
      const categoryCSS = `
/* ========================================
   Category Enhancement Styles
   ======================================== */

/* Navigation list items with category indicators */
.category-operations {
  border-left: 3px solid #4F46E5 !important;
  background: rgba(79, 70, 229, 0.08) !important;
  margin: 2px 0 !important;
  border-radius: 0 4px 4px 0 !important;
}

.category-types {
  border-left: 3px solid #059669 !important;
  background: rgba(5, 150, 105, 0.08) !important;
  margin: 2px 0 !important;
  border-radius: 0 4px 4px 0 !important;
}

.category-events {
  border-left: 3px solid #DC2626 !important;
  background: rgba(220, 38, 38, 0.08) !important;
  margin: 2px 0 !important;
  border-radius: 0 4px 4px 0 !important;
}

.category-primitives {
  border-left: 3px solid #7C2D12 !important;
  background: rgba(124, 45, 18, 0.08) !important;
  margin: 2px 0 !important;
  border-radius: 0 4px 4px 0 !important;
}

/* Category icons in navigation */
.category-icon {
  margin-right: 6px !important;
  font-size: 14px !important;
  display: inline-block !important;
  vertical-align: middle !important;
}

/* Category section headers */
.category-section {
  margin: 8px 0 !important;
  list-style: none !important;
  border-radius: 6px;
  overflow: hidden;
  background: var(--color-background-alt, #f8f9fa);
  contain: layout style paint !important; /* CSS containment for better performance */
}

.category-section-header {
  display: flex !important;
  justify-content: space-between !important;
  align-items: center !important;
  padding: 8px 12px !important;
  background: var(--color-background-alt, #f8f9fa);
  border: 1px solid var(--color-accent, #ddd);
  cursor: pointer !important;
  user-select: none !important;
  font-weight: 500 !important;
  font-size: 14px !important;
}

.category-section-header:hover {
  background: var(--color-background-hover, #e9ecef) !important;
}

.category-section-content {
  overflow: hidden !important;
  border: 1px solid var(--color-accent, #ddd);
  border-top: none;
  background: var(--color-background, white);
}

.category-section-content.collapsed {
  display: none !important;
}

.category-toggle {
  font-size: 12px !important;
  color: var(--color-text-aside, #666) !important;
}

.category-toggle.collapsed {
  transform: rotate(-90deg) !important;
}

/* Category specific colors */
.category-section-operations .category-section-header {
  border-left: 4px solid #007acc !important;
}

.category-section-types .category-section-header {
  border-left: 4px solid #28a745 !important;
}

.category-section-events .category-section-header {
  border-left: 4px solid #dc3545 !important;
}

.category-section-primitives .category-section-header {
  border-left: 4px solid #8b4513 !important;
}

/* Enhanced items */
.category-enhanced {
  /* Subtle enhancement that doesn't break layout */
}

.category-operations {
  /* Operations styling */
}

.category-types {
  /* Types styling */
}

.category-events {
  /* Events styling */
}

.category-primitives {
  /* Primitives styling */
}

/* Responsive design */
@media (max-width: 768px) {
  .category-section-header {
    padding: 6px 8px !important;
    font-size: 13px !important;
  }
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .category-section {
    background: var(--color-background-alt, #2d3748) !important;
  }
  
  .category-section-header {
    background: var(--color-background-alt, #2d3748) !important;
    border-color: var(--color-accent, #4a5568) !important;
    color: var(--color-text, #e2e8f0) !important;
  }
  
  .category-section-header:hover {
    background: var(--color-background-hover, #4a5568) !important;
  }
  
  .category-section-content {
    background: var(--color-background, #1a202c) !important;
    border-color: var(--color-accent, #4a5568) !important;
  }
}

/* Prevent layout shifts */
.category-section * {
  box-sizing: border-box !important;
}

/* Ensure proper z-index for dropdowns */
.category-section {
  position: relative !important;
  z-index: 1 !important;
}
`;

      // JavaScript for navigation enhancement
      const categoryJS = `
(function() {
  'use strict';
  
  let isEnhanced = false; // Flag to prevent multiple enhancements
  
  // Preloaded type categories from JSDoc
  const typeCategories = ${JSON.stringify(typeCategories, null, 2)};
  
  const categories = {
    Operations: { 
      icon: "⚙️"
    },
    Types: { 
      icon: "📋"
    },
    Events: { 
      icon: "📢"
    },
    Primitives: { 
      icon: "🧱"
    }
  };

  function categorizeElement(element) {
    const text = element.textContent || '';
    const typeName = text.trim();
    
    console.log('Attempting to categorize:', typeName);
    
    // ONLY use preloaded JSDoc categories for strict reliability
    if (typeCategories[typeName]) {
      console.log('✓ Matched "' + typeName + '" to', typeCategories[typeName], 'by preloaded JSDoc');
      return typeCategories[typeName];
    }
    
    console.log('✗ No preloaded JSDoc category for:', typeName);
    return null;
  }

  function enhanceNavigation() {
    // Prevent multiple enhancements
    if (isEnhanced) {
      console.log('Navigation already enhanced, skipping');
      return;
    }
    
    // Only look for links in navigation areas, not in main content
    const navContainers = document.querySelectorAll('.site-menu, .tsd-navigation');
    if (navContainers.length === 0) {
      console.log('No navigation containers found');
      return;
    }
    
    // Find all links in navigation areas that match our JSDoc categories
    const allNavLinks = [];
    navContainers.forEach(container => {
      const links = Array.from(container.querySelectorAll('a'));
      allNavLinks.push(...links);
    });
    
    console.log('Total navigation links found:', allNavLinks.length);
    
    // Debug: show all navigation link texts
    const allNavTexts = allNavLinks.map(link => link.textContent?.trim()).filter(Boolean);
    console.log('All navigation links:', allNavTexts.slice(0, 20), allNavTexts.length > 20 ? '...' : '');
    
    // Get all type names that have JSDoc categories
    const jsdocTypeNames = Object.keys(typeCategories);
    console.log('JSDoc type names:', jsdocTypeNames.length);
    console.log('JSDoc types:', jsdocTypeNames);
    
    // Debug: check for potential duplicates in preloaded categories
    const duplicateCheck = {};
    jsdocTypeNames.forEach(typeName => {
      if (duplicateCheck[typeName]) {
        console.warn('⚠️ Duplicate type in JSDoc categories:', typeName);
      } else {
        duplicateCheck[typeName] = true;
      }
    });
    
    let navLinks = allNavLinks.filter(link => {
      const text = link.textContent?.trim() || '';
      const isProcessed = link.hasAttribute('data-category-enhanced');
      const hasJSDocCategory = jsdocTypeNames.includes(text);
      
      // Debug: log filtering decision
      if (text && jsdocTypeNames.length > 0) {
        if (hasJSDocCategory) {
          console.log('✓ Including link:', text, '(has JSDoc category)');
        } else {
          console.log('✗ Excluding link:', text, '(no JSDoc category)');
        }
      }
      
      return hasJSDocCategory && !isProcessed;
    });
    
    console.log('Found JSDoc navigation links to enhance:', navLinks.length);
    
    if (navLinks.length === 0) {
      console.log('No unprocessed JSDoc navigation links found for enhancement');
      return;
    }
    
    // Log some example links for debugging
    navLinks.slice(0, 3).forEach(link => {
      console.log('Example target link:', link.textContent.trim());
    });
    
    console.log('Enhancing TypeDoc navigation with categories');
    isEnhanced = true;
    
    const categoryGroups = {};
    const processedTypes = {}; // Track processed types to avoid duplicates
    
    navLinks.forEach(link => {
      // Mark as processed
      link.setAttribute('data-category-enhanced', 'true');
      
      const category = categorizeElement(link);
      if (category) {
        const typeName = link.textContent?.trim();
        const categoryKey = category + ':' + typeName;
        
        // Skip if this type was already added to this category
        if (processedTypes[categoryKey]) {
          console.log('⚠️ Skipping duplicate:', typeName, 'in', category);
          
          // Still apply visual enhancements to duplicates, but don't add to groups
          const listItem = link.closest('li');
          if (listItem && !listItem.classList.contains('category-enhanced')) {
            listItem.classList.add('category-' + category.toLowerCase());
            listItem.classList.add('category-enhanced');
          }
          
          return;
        }
        
        processedTypes[categoryKey] = true;
        
        if (!categoryGroups[category]) {
          categoryGroups[category] = [];
        }
        
        // Debug: log each type that gets categorized
        console.log('✓ Adding to ' + category + ':', typeName);
        
        // Add visual enhancements to the link (without icons)
        const listItem = link.closest('li');
        if (listItem && !listItem.classList.contains('category-enhanced')) {
          listItem.classList.add('category-' + category.toLowerCase());
          listItem.classList.add('category-enhanced');
        }
        
        categoryGroups[category].push(link.parentElement || link);
      }
    });

    // Log categories found
    Object.keys(categoryGroups).forEach(category => {
      console.log('Found ' + categoryGroups[category].length + ' items in ' + category + ' category');
      
      // Debug: show all items in this category
      const itemNames = categoryGroups[category].map(function(item) {
        const link = item.querySelector('a') || item;
        return link.textContent?.trim() || 'unknown';
      });
      console.log('→ ' + category + ' items:', itemNames);
    });
    
    // Final verification: check if counts match preloaded data
    const expectedCounts = {};
    Object.values(typeCategories).forEach(category => {
      expectedCounts[category] = (expectedCounts[category] || 0) + 1;
    });
    
    console.log('Expected category counts:', expectedCounts);
    
    // Debug: show expected types per category
    Object.keys(expectedCounts).forEach(category => {
      const typesInCategory = Object.keys(typeCategories).filter(type => typeCategories[type] === category);
      console.log('→ Expected ' + category + ' types:', typesInCategory);
    });
    
    console.log('Actual category counts:', Object.fromEntries(
      Object.entries(categoryGroups).map(function(entry) {
        return [entry[0], entry[1].length];
      })
    ));
    
    // Warn if there are mismatches
    Object.keys(categoryGroups).forEach(category => {
      const expected = expectedCounts[category] || 0;
      const actual = categoryGroups[category].length;
      if (expected !== actual) {
        console.warn('⚠️ Category ' + category + ' mismatch: expected ' + expected + ', got ' + actual);
      }
    });

    // Create category sections in appropriate navigation containers
    const categoryEntries = Object.entries(categoryGroups);
    console.log('About to process', categoryEntries.length, 'categories:', categoryEntries.map(([name]) => name));
    
    // First pass: prepare all elements for each category (no DOM manipulation yet)
    const categoryElements = {};
    
    for (let i = 0; i < categoryEntries.length; i++) {
      const [categoryName, elements] = categoryEntries[i];
      console.log('=== PREPARING CATEGORY', i + 1, '/', categoryEntries.length, ':', categoryName, '===');
      
      if (elements.length === 0) {
        console.log('Skipping empty category:', categoryName);
        categoryElements[categoryName] = [];
        continue;
      }
      
      const validElements = [];
      
      for (let j = 0; j < elements.length; j++) {
        const element = elements[j];
        console.log('Preparing element', j + 1, '/', elements.length, 'for category', categoryName);
        
        try {
          console.log('🔍 About to call element.closest for element', j + 1, 'in category', categoryName);
          const listItem = element.closest('li');
          console.log('✅ Called element.closest successfully for element', j + 1, 'in category', categoryName);
          
          if (listItem && !listItem.classList.contains('category-section')) {
            console.log('🔍 About to push element to validElements for element', j + 1, 'in category', categoryName);
            console.log('🔍 Element textContent:', listItem.textContent?.trim());
            
            validElements.push({
              element: listItem,
              name: listItem.textContent?.trim() || 'unknown'
            });
            
            console.log('✅ Pushed element to validElements successfully for element', j + 1, 'in category', categoryName);
            console.log('✓ Prepared element', j + 1, ':', listItem.textContent?.trim());
          } else {
            console.warn('✗ Invalid element', j + 1, 'for category', categoryName, '- no list item or is category section');
            if (!listItem) {
              console.warn('   - listItem is null/undefined');
            } else if (listItem.classList.contains('category-section')) {
              console.warn('   - listItem has category-section class');
            }
          }
        } catch (error) {
          console.error('❌ Error preparing element', j + 1, 'for category', categoryName, ':', error);
          console.error('Error stack:', error.stack);
        }
        
        console.log('Completed preparing element', j + 1, 'for category', categoryName);
      }
      
      console.log('✅ Finished preparing all elements for category:', categoryName);
      categoryElements[categoryName] = validElements;
      console.log('📊 Prepared', validElements.length, 'valid elements for category:', categoryName);
    }
    
    // Second pass: create category sections and move elements
    categoryEntries.forEach(([categoryName, elements], categoryIndex) => {
      console.log('=== STARTING CATEGORY', categoryIndex + 1, '/', categoryEntries.length, ':', categoryName, '===');
      
      const validElements = categoryElements[categoryName];
      if (!validElements || validElements.length === 0) {
        console.log('No valid elements for category:', categoryName);
        return;
      }
      
      console.log('Creating category section for:', categoryName, 'with', validElements.length, 'elements');
      
      const categoryInfo = categories[categoryName];
      const firstElement = validElements[0].element;
      const parentList = firstElement.closest('ul');
      const navContainer = firstElement.closest('.site-menu, .tsd-navigation');
      
      if (parentList && navContainer && !parentList.querySelector('.category-section-' + categoryName.toLowerCase())) {
        // Create section header
        const sectionHeader = document.createElement('li');
        sectionHeader.className = 'category-section category-section-' + categoryName.toLowerCase();
        
        const header = document.createElement('div');
        header.className = 'category-section-header';
        header.innerHTML = 
          '<span>' + categoryInfo.icon + ' ' + categoryName + ' (' + validElements.length + ')</span>' +
          '<span class="category-toggle">▼</span>';
        
        const content = document.createElement('div');
        content.className = 'category-section-content';
        
        // Optimize for large categories
        if (validElements.length > 20) {
          content.style.maxHeight = '300px';
          content.style.overflowY = 'auto';
          content.style.scrollbarWidth = 'thin';
        }
        
        // Move elements to the content using DocumentFragment for better performance
        const fragment = document.createDocumentFragment();
        let movedCount = 0;
        
        console.log('Starting to move elements for category:', categoryName);
        
        const cycleId = Date.now() + '-' + Math.random().toString(36).substr(2, 9);
        console.log('forEach cycle ID for', categoryName, ':', cycleId);
        
        validElements.forEach(function(element, index) {
          console.log('[' + cycleId + '] Processing element', index + 1, '/', validElements.length, 'for category', categoryName);
          
          try {
            const listItem = element.element;
            if (listItem && listItem !== sectionHeader && !listItem.classList.contains('category-section')) {
              console.log('[' + cycleId + '] Moving element', index + 1, '/', validElements.length, ':', listItem.textContent?.trim());
              
              // Check if element is still in DOM
              if (listItem.parentNode) {
                fragment.appendChild(listItem);
                movedCount++;
                console.log('[' + cycleId + '] Successfully appended to fragment:', listItem.textContent?.trim());
              } else {
                console.warn('[' + cycleId + '] Element not in DOM, skipping:', listItem.textContent?.trim());
              }
            } else {
              console.warn('[' + cycleId + '] Could not find valid list item for element', index + 1, ':', element);
            }
          } catch (error) {
            console.error('[' + cycleId + '] Error moving element', index + 1, 'for category', categoryName, ':', error);
            console.error('[' + cycleId + '] Error stack:', error.stack);
          }
          
          console.log('[' + cycleId + '] Completed processing element', index + 1, 'for category', categoryName);
        });
        
        console.log('[' + cycleId + '] Finished moving elements. Successfully moved', movedCount, '/', validElements.length, 'elements to', categoryName, 'category');
        
        // Append fragment to content
        try {
          content.appendChild(fragment);
          console.log('[' + cycleId + '] Successfully appended fragment to content for category:', categoryName);
        } catch (error) {
          console.error('[' + cycleId + '] Error appending fragment to content:', error);
        }
        
        sectionHeader.appendChild(header);
        sectionHeader.appendChild(content);
        
        // Add collapse functionality - instant toggle
        header.addEventListener('click', function(e) {
          e.preventDefault();
          e.stopPropagation();
          
          const toggle = this.querySelector('.category-toggle');
          content.classList.toggle('collapsed');
          toggle.classList.toggle('collapsed');
          
          // Save state
          const isCollapsed = content.classList.contains('collapsed');
          localStorage.setItem('typedoc-category-' + categoryName, isCollapsed.toString());
        });
        
        // Restore state
        const savedState = localStorage.getItem('typedoc-category-' + categoryName);
        if (savedState === 'true') {
          content.classList.add('collapsed');
          header.querySelector('.category-toggle').classList.add('collapsed');
        }
        
        // Insert at the beginning of the parent list
        parentList.insertBefore(sectionHeader, parentList.firstChild);
        
        console.log('=== COMPLETED CATEGORY', categoryIndex + 1, '/', categoryEntries.length, ':', categoryName, '===');
      } else {
        console.log('Could not create category section for:', categoryName, '- no parent list or nav container');
      }
    });
  }

  // Simple initialization without MutationObserver to avoid loops
  function initialize() {
    console.log('Initializing category enhancement');
    
    // Try multiple times with delays to catch dynamically loaded content
    let attempts = 0;
    const maxAttempts = 5; // Reduced attempts for faster response
    
    function tryEnhance() {
      attempts++;
      
      // Count links with JSDoc categories in navigation areas only
      const navContainers = document.querySelectorAll('.site-menu, .tsd-navigation');
      if (navContainers.length === 0) {
        console.log(\`Attempt \${attempts}/\${maxAttempts}: No navigation containers found\`);
        if (attempts < maxAttempts) {
          setTimeout(tryEnhance, 200 * attempts);
        }
        return;
      }
      
      const allNavLinks = [];
      navContainers.forEach(container => {
        const links = Array.from(container.querySelectorAll('a'));
        allNavLinks.push(...links);
      });
      
      const jsdocTypeNames = Object.keys(typeCategories);
      
      const targetLinks = allNavLinks.filter(link => {
        const text = link.textContent?.trim() || '';
        return jsdocTypeNames.includes(text);
      });
      
      console.log(\`Attempt \${attempts}/\${maxAttempts}: Found \${targetLinks.length} JSDoc type links in navigation\`);
      
      if (targetLinks.length > 0 && !isEnhanced) {
        enhanceNavigation();
      } else if (attempts < maxAttempts) {
        console.log(\`Retrying in \${200 * attempts}ms...\`);
        setTimeout(tryEnhance, 200 * attempts); // Faster retry
      } else {
        console.log('Max attempts reached, giving up');
      }
    }
    
    tryEnhance();
  }

  // Reset enhancement state when navigating
  function resetEnhancement() {
    console.log('Resetting enhancement state');
    isEnhanced = false;
    
    // Only remove existing category sections from sidebar/navigation areas
    const existingSections = document.querySelectorAll('.site-menu .category-section, .tsd-navigation .category-section');
    existingSections.forEach(section => {
      console.log('Removing category section:', section.className);
      section.remove();
    });
    
    // Only reset category markers in navigation areas, not in main content
    const navContainers = document.querySelectorAll('.site-menu, .tsd-navigation');
    navContainers.forEach(container => {
      const enhancedLinks = container.querySelectorAll('[data-category-enhanced]');
      enhancedLinks.forEach(link => {
        link.removeAttribute('data-category-enhanced');
      });
      
      // Remove category classes from navigation items only
      const enhancedItems = container.querySelectorAll('.category-enhanced');
      enhancedItems.forEach(item => {
        item.classList.remove('category-enhanced');
        // Remove category-specific classes
        ['operations', 'types', 'events', 'primitives'].forEach(cat => {
          item.classList.remove('category-' + cat);
        });
      });
    });
    
    console.log('Enhancement reset completed');
  }

  // Enhanced MutationObserver for better TypeDoc navigation detection
  function setupNavigationObserver() {
    let observerTimeout = null;
    let navigationTimeout = null;
    let lastUrl = window.location.href;
    
    const observer = new MutationObserver(function(mutations) {
      // Debounce to prevent excessive calls
      if (observerTimeout) return;
      
      observerTimeout = setTimeout(() => {
        observerTimeout = null;
        
        // Check if URL actually changed (real navigation)
        const currentUrl = window.location.href;
        const urlChanged = currentUrl !== lastUrl;
        
        if (!urlChanged) {
          // No navigation occurred, just content updates
          return;
        }
        
        console.log('URL navigation detected:', lastUrl, '->', currentUrl);
        lastUrl = currentUrl;
        
        // Check for significant content changes that indicate page load
        const hasSignificantChanges = mutations.some(mutation => {
          if (mutation.type === 'childList') {
            return Array.from(mutation.addedNodes).some(node => {
              if (node.nodeType === Node.ELEMENT_NODE) {
                // Look for major page structure changes
                return node.matches?.('.tsd-page-title, .tsd-breadcrumb, main .container') ||
                       node.querySelector?.('.tsd-page-title, .tsd-breadcrumb, main .container');
              }
              return false;
            });
          }
          return false;
        });
        
        if (hasSignificantChanges) {
          console.log('Significant page changes detected');
          
          // Clear any existing navigation timeout
          if (navigationTimeout) {
            clearTimeout(navigationTimeout);
          }
          
          // Only reset if we're navigating to a different type of page
          navigationTimeout = setTimeout(() => {
            // Check if current page has navigation sidebar
            const hasSidebar = document.querySelector('.site-menu, .tsd-navigation');
            if (hasSidebar && !isEnhanced) {
              console.log('Reinitializing categories for new page');
              initialize();
            } else if (hasSidebar && isEnhanced) {
              console.log('Page with sidebar loaded, categories already active');
              // Just verify categories are still there, don't reset
              const existingSections = document.querySelectorAll('.category-section');
              if (existingSections.length === 0) {
                console.log('Categories missing, reinitializing');
                resetEnhancement();
                initialize();
              }
            }
          }, 150);
        }
      }, 100);
    });

    observer.observe(document.body, { 
      childList: true, 
      subtree: true,
      attributes: false // Reduce noise
    });
    
    return observer;
  }

  // Handle browser navigation (back/forward buttons)
  function setupHistoryListener() {
    window.addEventListener('popstate', function(event) {
      console.log('Browser navigation detected (popstate)');
      setTimeout(() => {
        const hasSidebar = document.querySelector('.site-menu, .tsd-navigation');
        if (hasSidebar) {
          resetEnhancement();
          initialize();
        }
      }, 150);
    });
  }

  // Handle TypeDoc navigation clicks with better detection
  function setupClickListener() {
    document.addEventListener('click', function(event) {
      const link = event.target.closest('a');
      if (link && link.href && !link.href.startsWith('http') && !link.href.includes('#')) {
        // Only handle internal navigation, not anchors
        const currentUrl = window.location.href;
        const targetUrl = link.href;
        
        if (currentUrl !== targetUrl) {
          console.log('Internal navigation click detected');
          
          // Set up a one-time listener for when navigation completes
          setTimeout(() => {
            let checkCount = 0;
            const maxChecks = 10;
            
            function checkNavigationComplete() {
              checkCount++;
              const newUrl = window.location.href;
              
              if (newUrl === targetUrl) {
                console.log('Navigation completed to:', newUrl);
                const hasSidebar = document.querySelector('.site-menu, .tsd-navigation');
                if (hasSidebar) {
                  resetEnhancement();
                  initialize();
                }
              } else if (checkCount < maxChecks) {
                setTimeout(checkNavigationComplete, 50);
              }
            }
            
            checkNavigationComplete();
          }, 50);
        }
      }
    });
  }

  // Start initialization
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', function() {
      initialize();
      setupNavigationObserver();
      setupHistoryListener();
      setupClickListener();
    });
  } else {
    initialize();
    setupNavigationObserver();
    setupHistoryListener();
    setupClickListener();
  }

})();
`;

      // Write CSS file
      const cssPath = path.join(assetsDir, "category-styles.css");
      fs.writeFileSync(cssPath, categoryCSS);
      console.log("Created category CSS at:", cssPath);

      // Write JS file
      const jsPath = path.join(assetsDir, "category-behavior.js");
      fs.writeFileSync(jsPath, categoryJS);
      console.log("Created category JS at:", jsPath);

      // Modify HTML files to include the CSS and JS
      modifyHtmlFiles(outputDir);

      console.log("Category enhancement completed successfully");
    } catch (error) {
      console.error("❌ Error in category plugin:", error);
    }
  });
}

function modifyHtmlFiles(outputDir) {
  try {
    // Get all HTML files
    const htmlFiles = getAllHtmlFiles(outputDir);
    console.log(`Modifying ${htmlFiles.length} HTML files`);

    htmlFiles.forEach((htmlFile) => {
      let content = fs.readFileSync(htmlFile, "utf8");
      let modified = false;

      const relativePath = path.relative(path.dirname(htmlFile), outputDir);
      const assetsPath = relativePath ? `${relativePath}/assets` : "./assets";

      // Add CSS link before closing head tag
      if (!content.includes("category-styles.css") && content.includes("</head>")) {
        const cssLink = `  <link rel="stylesheet" href="${assetsPath}/category-styles.css">\n`;
        content = content.replace("</head>", cssLink + "</head>");
        modified = true;
      }

      // Add JS script before closing body tag
      if (!content.includes("category-behavior.js") && content.includes("</body>")) {
        const jsScript = `  <script src="${assetsPath}/category-behavior.js"></script>\n`;
        content = content.replace("</body>", jsScript + "</body>");
        modified = true;
      }

      if (modified) {
        fs.writeFileSync(htmlFile, content);
      }
    });
  } catch (error) {
    console.error("❌ Error modifying HTML files:", error);
  }
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

function preloadTypeCategories() {
  const categories = {};
  const rootDir = path.resolve(__dirname, "../../");

  // Read typedoc.json to get entry points
  const typedocConfigPath = path.join(rootDir, "typedoc.json");
  let entryPoints = [];

  try {
    const typedocConfig = JSON.parse(fs.readFileSync(typedocConfigPath, "utf8"));
    entryPoints = typedocConfig.entryPoints || [];
    console.log("Found entry points in typedoc.json:", entryPoints.length);
  } catch (error) {
    console.warn("Failed to read typedoc.json:", error.message);
    return categories;
  }

  // Recursively find all TypeScript files in entry points
  function findTypeScriptFiles(dir) {
    const tsFiles = [];

    try {
      const items = fs.readdirSync(dir);

      for (const item of items) {
        const fullPath = path.join(dir, item);

        try {
          const stat = fs.statSync(fullPath);

          if (stat.isDirectory()) {
            // Skip node_modules and hidden directories
            if (!item.startsWith(".") && item !== "node_modules") {
              tsFiles.push(...findTypeScriptFiles(fullPath));
            }
          } else if (stat.isFile() && item.endsWith(".ts") && !item.endsWith(".d.ts")) {
            // Include TypeScript files, exclude declaration files
            tsFiles.push(fullPath);
          }
        } catch (statError) {
          // Skip files that can't be stat'd (permissions, broken symlinks, etc.)
          // Only log if it's actually a TypeScript file we expected to process
          if (item.endsWith(".ts")) {
            console.warn(`Could not stat TypeScript file: ${fullPath} - ${statError.message}`);
          }
        }
      }
    } catch (error) {
      console.warn("Failed to read directory:", dir, error.message);
    }

    return tsFiles;
  }

  let totalFiles = 0;
  let processedFiles = 0;

  // Scan all TypeScript files in each entry point
  entryPoints.forEach((entryPoint) => {
    const entryPointPath = path.join(rootDir, entryPoint);

    if (fs.existsSync(entryPointPath)) {
      const tsFiles = findTypeScriptFiles(entryPointPath);
      totalFiles += tsFiles.length;

      console.log(`Found ${tsFiles.length} TypeScript files in ${entryPoint}`);

      tsFiles.forEach((filePath) => {
        try {
          const content = fs.readFileSync(filePath, "utf8");
          const typeCategories = extractJSDocCategories(content);

          if (Object.keys(typeCategories).length > 0) {
            Object.assign(categories, typeCategories);
            processedFiles++;

            // Log which file contributed categories
            const relativePath = path.relative(rootDir, filePath);
            console.log(`Found categories in: ${relativePath} (${Object.keys(typeCategories).length} types)`);
          }
        } catch (error) {
          console.warn("Failed to read", path.relative(rootDir, filePath), ":", error.message);
        }
      });
    } else {
      console.warn(`Entry point not found: ${entryPoint}`);
    }
  });

  console.log(`Scanned ${totalFiles} TypeScript files, found categories in ${processedFiles} files`);
  return categories;
}

function extractJSDocCategories(content) {
  const categories = {};

  // Match JSDoc comments with @category tag followed by export statements
  const pattern = /\/\*\*[\s\S]*?@category\s+(\w+)[\s\S]*?\*\/\s*export\s+type\s+(\w+)/g;
  let match;

  while ((match = pattern.exec(content)) !== null) {
    const [, category, typeName] = match;
    categories[typeName] = category;
  }

  return categories;
}

module.exports = { load };
