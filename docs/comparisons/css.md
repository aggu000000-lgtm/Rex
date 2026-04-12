# Rex vs CSS: The Styling Model

## The Limitations of CSS

CSS (Cascading Style Sheets) was originally intended to apply styles to plain text documents. When repurposed for complex application layouts, developers must navigate a variety of historically complex systems and rulesets:

- **The Box Model**: Margins, padding, borders, content area, and margin collapse interactions.
- **The Cascade and Specificity**: Managing rule precedence (inline > ID > class > element) and resolving conflicting styles.
- **Positioning and Contexts**: Static, relative, absolute, fixed, and sticky positioning, plus understanding z-index stacking contexts.
- **Layout Engines**: Flexbox (main/cross axes, flex-wrap, justify-content) and CSS Grid (fr units, templates, minmax).
- **Miscellaneous Additions**: Media queries, pseudo-classes, transitions, animations, and variable handling.

This inherent complexity often requires third-party preprocessors (SASS, LESS) or libraries (Tailwind, Styled Components) to manage large-scale styling reliably.

## The Rex Styling Model

Rex takes a fundamentally different approach. Instead of cascading rules applied across elements using selectors, Rex properties apply strictly and exactly to the element they are declared on. There is no cascade, no specificity conflicts, and no `!important` declarations.

In Rex, styles read as direct instructions without complex units or verbose syntax.

| Desired Style | Rex Syntax |
|---|---|
| Large text | `size is 32` |
| Bold text | `bold` |
| Space between elements | `gap is 16` |
| Internal padding | `padding is 24` |
| Rounded corners | `rounded` |
| Drop shadow | `shadow` |
| Center content | `centered` |
| Expand to full width | `stretched` |
| Hide an element | `hidden` |

There are no conflicting decisions regarding `px`, `em`, `rem`, `vh`, or `%` units. You describe the visual intent, and Rex renders it natively.

### Built-in Color Palette

Working with colors in standard CSS involves hex codes, rgb(), or hsl(), which can be difficult to read and standardize across a project without external design tokens.

```css
/* Traditional CSS color declarations */
color: #6366f1;
color: rgb(99, 102, 241);
color: hsl(239, 84%, 67%);
color: indigo; /* The default browser representation may vary or conflict with brand palettes */
```

Rex incorporates a built-in palette of more than 20 named colors, each natively supporting `dark` and `light` variants. The colors are designed to be visually appealing, consistent across platforms, and syntactically clear.

```rex
# Rex color declarations
color is indigo
color is dark indigo
color is light indigo
color is violet
color is dark navy
color is light coral
```

Developers can consistently reference beautiful colors without directly managing hex code mappings.
