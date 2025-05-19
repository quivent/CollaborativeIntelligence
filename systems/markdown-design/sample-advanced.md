---
title: "Advanced Markdown Features"
description: "A showcase of advanced markdown styling and components"
theme: "ocean"
layout: "toc"
author: "Designer Agent"
siteUrl: "/index.html"
siteName: "Markdown Design System"
---

# Advanced Markdown Features

This document showcases the enhanced capabilities of the Advanced Markdown Design System, featuring interactive components, advanced styling, and flexible layouts.

## Introduction

The Advanced Markdown Design System transforms basic markdown into beautifully styled, feature-rich HTML documents. With this system, you can create documentation, articles, reports, and other content with minimal effort but maximum impact.

:::note
This is an admonition block - a special callout that draws attention to important information. You'll find several types of these throughout this document.
:::

## Core Features

The system includes numerous enhancements over standard markdown:

### Component System

The Advanced Markdown Design System includes pre-built components that can be used directly in your markdown:

- **Admonitions** - Callout boxes for notes, warnings, tips, etc.
- **Tabs** - Organize content into tabbed sections
- **Accordions** - Collapsible content sections
- **Cards** - Content containers with consistent styling
- **Alerts** - Contextual feedback messages
- **Badges** - Small count and labeling components

### Advanced Styling

Extensive styling options are available:

| Feature | Description | Example |
|---------|-------------|---------|
| Color Themes | Multiple built-in color schemes | Light, Dark, Sepia, Minimal, Ocean |
| Layout Options | Different content arrangements | Centered, Sidebar, TOC layouts |
| Typography | Carefully selected fonts | Headings, body text, code blocks |
| Animations | Subtle motion effects | Slide, fade, reveal animations |
| Components | Interactive UI elements | Tabs, accordions, tooltips |

### Code Highlighting

Code blocks are automatically syntax highlighted:

```javascript
// Sample JavaScript with highlighting
function calculateTotal(items) {
  return items
    .filter(item => item.price > 0)
    .map(item => item.price * item.quantity)
    .reduce((total, price) => total + price, 0);
}

// Example usage
const cart = [
  { name: 'Widget', price: 9.99, quantity: 2 },
  { name: 'Gadget', price: 14.99, quantity: 1 },
];
const total = calculateTotal(cart);
console.log(`Total: $${total.toFixed(2)}`);
```

## Interactive Components

### Tabs Component

====Tabs====
JavaScript: 
```javascript
function greet(name) {
  return `Hello, ${name}!`;
}
```

Python: 
```python
def greet(name):
    return f"Hello, {name}!"
```

Ruby: 
```ruby
def greet(name)
  "Hello, #{name}!"
end
```
====End====

### Accordion Component

^^^^What is the Markdown Design System?^^^^
The Markdown Design System is a comprehensive solution for converting standard markdown files into beautifully styled, feature-rich HTML documents. It includes templates, styling, and JavaScript functionality to enhance the presentation of your content.
^^^^End^^^^

^^^^How do I use it?^^^^
Simply write your content in markdown format, then process it using the provided conversion script. The system will handle all the styling and interactivity for you.

```bash
./scripts/convert-enhanced.sh your-document.md
```
^^^^End^^^^

^^^^Can I customize it?^^^^
Yes! The system is designed to be highly customizable:

1. Modify the CSS to change styling
2. Edit the template HTML to change layout
3. Extend the JavaScript functionality for custom components
4. Add new themes to the existing collection
^^^^End^^^^

## Alert Types

<div class="alert alert-info">
This is an informational alert that provides context or additional details.
</div>

<div class="alert alert-success">
This is a success alert confirming that an action was completed successfully.
</div>

<div class="alert alert-warning">
This is a warning alert notifying about potential issues that require attention.
</div>

<div class="alert alert-danger">
This is a danger alert indicating critical problems that need immediate action.
</div>

## Badge Examples

Feature status: <span class="badge badge-primary">New</span>
Version: <span class="badge badge-secondary">v2.0.0</span>
Build: <span class="badge badge-success">Passing</span>
Issues: <span class="badge badge-warning">4</span>
Critical bugs: <span class="badge badge-danger">0</span>
Documentation: <span class="badge badge-info">Updated</span>

## Card Components

<div class="card-grid">
  <div class="card">
    <h3 class="card-title">Getting Started</h3>
    <p>Quick introduction to the Markdown Design System and how to set it up for your project.</p>
    <div class="card-footer">
      <a href="#" class="btn btn-primary">Learn More</a>
    </div>
  </div>
  
  <div class="card">
    <h3 class="card-title">Advanced Features</h3>
    <p>Explore the advanced capabilities that set this system apart from basic markdown converters.</p>
    <div class="card-footer">
      <a href="#" class="btn btn-primary">Discover</a>
    </div>
  </div>
  
  <div class="card">
    <h3 class="card-title">Customization</h3>
    <p>Learn how to tailor the system to your specific needs with custom themes and components.</p>
    <div class="card-footer">
      <a href="#" class="btn btn-primary">Customize</a>
    </div>
  </div>
</div>

## Admonition Types

:::note
This is a standard note for providing additional context or information.
:::

:::warning
This is a warning to alert users about potential issues or pitfalls.
:::

:::danger
This is a danger admonition to indicate serious problems or critical information.
:::

:::tip
This is a tip providing helpful advice or shortcuts.
:::

:::important
This is highlighting particularly important information that shouldn't be overlooked.
:::

## Button Styles

<div class="btn-group">
  <button class="btn">Default</button>
  <button class="btn btn-primary">Primary</button>
  <button class="btn btn-secondary">Secondary</button>
  <button class="btn btn-success">Success</button>
  <button class="btn btn-warning">Warning</button>
  <button class="btn btn-danger">Danger</button>
  <button class="btn btn-info">Info</button>
</div>

<div class="btn-group mt-3">
  <button class="btn btn-outline">Outline</button>
  <button class="btn btn-sm">Small</button>
  <button class="btn btn-lg">Large</button>
</div>

## Image Enhancements

Images can be enhanced with captions, shadows, and borders:

<figure>
  <img src="https://via.placeholder.com/800x400?text=Sample+Image" alt="Sample image" class="img-shadow">
  <figcaption>A sample image with shadow effect</figcaption>
</figure>

## Typography Examples

# Heading 1 (for document titles)
## Heading 2 (for major sections)
### Heading 3 (for subsections)
#### Heading 4 (for smaller divisions)
##### Heading 5 (for fine-grained elements)
###### Heading 6 (for the smallest sections)

Standard paragraph text looks like this. It uses the system's body font, with careful attention to line height, contrast, and readability.

**Bold text** for emphasis and strong statements.

*Italic text* for terms, book titles, or mild emphasis.

***Bold italic text*** for the strongest emphasis.

~~Strikethrough text~~ for outdated or removed content.

<mark>Highlighted text</mark> to draw attention to specific words or phrases.

<span class="text-primary">Colored text</span> for additional visual distinction.

## Blockquote Styling

> "The Advanced Markdown Design System makes documentation beautiful, interactive, and enjoyable to read. It transforms simple text into rich, engaging experiences."
>
> <cite>Design Team</cite>

## Horizontal Rule Variations

Standard horizontal rule:

---

Fancy horizontal rule with center element:

<hr class="fancy">

## Responsive Features

The system is fully responsive and works beautifully on devices of all sizes:

- **Mobile optimization** - Perfect reading experience on smartphones
- **Tablet-friendly** - Optimized layouts for medium-sized screens
- **Desktop excellence** - Takes advantage of larger screen real estate
- **Print styling** - Properly formatted for printing or PDF export

## Animation Examples

<div class="animate-fade-in">
  <p>This content fades in when the page loads.</p>
</div>

<div class="animate-slide-up">
  <p>This content slides up from below.</p>
</div>

## Utility Classes

The system includes numerous utility classes for common styling needs:

```html
<!-- Text alignment -->
<div class="text-center">Centered text</div>
<div class="text-right">Right-aligned text</div>

<!-- Margins and spacing -->
<div class="mt-4">Top margin</div>
<div class="mb-3">Bottom margin</div>

<!-- Display utilities -->
<div class="d-none d-md-block">Hidden on small screens</div>

<!-- Flexbox utilities -->
<div class="d-flex justify-content-between">
  <div>Left content</div>
  <div>Right content</div>
</div>
```

## Tooltips Example

Hover over <span class="tooltip">this text<span class="tooltip-text">This is a tooltip that appears on hover!</span></span> to see a tooltip.

## Theme Switching

The system includes multiple built-in themes:

- **Light theme** - Clean and crisp for daytime reading
- **Dark theme** - Easy on the eyes for nighttime viewing
- **Sepia theme** - Warm tones for comfortable extended reading
- **Minimal theme** - Distraction-free simplicity
- **Ocean theme** - Calming blue tones

## Definition List Example

<dl>
  <dt>Markdown</dt>
  <dd>A lightweight markup language with plain text formatting syntax.</dd>
  
  <dt>HTML</dt>
  <dd>HyperText Markup Language, the standard language for creating web pages.</dd>
  
  <dt>CSS</dt>
  <dd>Cascading Style Sheets, a style sheet language used for describing the presentation of a document.</dd>
</dl>

## Conclusion

The Advanced Markdown Design System provides a comprehensive solution for creating beautiful, interactive documentation. With its rich feature set, extensive customization options, and thoughtful design, it transforms simple markdown into engaging, professional documents.

:::tip
Ready to get started? Check out the [documentation](https://example.com) or [installation guide](https://example.com/install).
:::

---

<footer>
  Created with the Advanced Markdown Design System by Designer Agent.
</footer>