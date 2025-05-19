# Sample Documentation

## Introduction

This is a sample markdown file to demonstrate the conversion capabilities of the Designer agent's markdown-to-HTML system. The system transforms plain markdown into elegantly styled HTML pages.

### Key Features

- **Clean, Responsive Design**: Looks great on all devices
- **Multiple Theme Options**: Light, dark, sepia, minimal, and ocean themes
- **Code Syntax Highlighting**: Makes code blocks beautiful and readable
- **Table of Contents**: Automatically generated for easy navigation
- **Elegant Typography**: Carefully selected fonts and spacing

## Code Examples

Here's a sample code block with syntax highlighting:

```javascript
// Sample JavaScript code
function convertMarkdown(text) {
  if (!text) return '';
  
  const html = marked(text);
  return html;
}

// Process a file
const result = convertMarkdown(fileContents);
console.log(`Converted ${result.length} characters`);
```

### Terminal Commands

You can convert markdown files using the provided script:

```bash
./convert.sh document.md
```

With options:

```bash
./convert.sh --theme dark --no-toc document.md
```

## Tables

Here's a sample table to demonstrate styling:

| Feature | Description | Status |
|---------|-------------|--------|
| Templates | HTML templates for markdown conversion | ✅ Completed |
| CSS Styling | Elegant CSS framework for markdown | ✅ Completed |
| Conversion Logic | Core markdown-to-HTML functionality | ✅ Completed |
| Themes | Multiple visual themes | ✅ Completed |
| Customization | User-configurable options | ✅ Completed |

## Blockquotes and Admonitions

> This is a standard blockquote. It's perfect for highlighting important information or quoting sources in your documentation.

Here's an example of an admonition block:

:::note
This is an important note. Admonition blocks make it easy to highlight different types of information.
:::

:::warning
Be careful with this approach! Warning blocks help alert users to potential issues.
:::

## Lists

### Unordered List

- First item
- Second item
- Third item with a longer description that might wrap to multiple lines to demonstrate how list formatting handles longer content
- Fourth item

### Ordered List

1. First step in the process
2. Second step in the process
3. Third step with sub-steps:
   1. Sub-step one
   2. Sub-step two
4. Final step

## Images

Images are displayed with elegant styling:

![Sample image placeholder](https://via.placeholder.com/800x400?text=Sample+Image)

## Mathematical Equations

The system can render LaTeX equations:

$$
f(x) = \int_{-\infty}^{\infty} \hat{f}(\xi) e^{2\pi i \xi x} d\xi
$$

## Collapsible Sections

<details>
<summary>Click to expand additional information</summary>

This content is hidden by default but can be expanded by clicking on the header.

- Hidden item 1
- Hidden item 2
- Hidden item 3

</details>

## Custom Styling

The system supports custom CSS styling for specific documents through frontmatter configuration.

## Conclusion

This sample demonstrates how the markdown-to-HTML conversion system transforms basic markdown into beautifully styled, professional documentation pages. The system is designed to be flexible, extensible, and produce elegant results with minimal configuration.