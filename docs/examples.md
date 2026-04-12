# Examples: Building a Dashboard

## Comparing Approaches

Building a comprehensive dashboard highlights the efficiency and readability advantages of Rex over the traditional web stack.

### The HTML/CSS/JS Approach

Creating a typical dashboard usually involves separating concerns across multiple files:
- **`index.html`** (approx. 85 lines): Deeply nested `div` elements, classes, and IDs for the structure.
- **`styles.css`** (approx. 210 lines): Styles for layout, flexbox configurations, media queries, hover states, and design system variables.
- **`app.js`** (approx. 140 lines): Logic for data fetching, DOM manipulation, event listeners, and state management.

**Total effort:** ~435 lines spread across 3 files.

### The Rex Approach

In Rex, the same dashboard layout, styling, and basic structure are consolidated into a single `.rex` file. The declarative syntax allows the entire component to read naturally, clearly outlining the layout hierarchy (columns, rows, cards) without external CSS or JS dependencies.

```rex
app "Dashboard":
    window size is 1200 by 800
    background is light gray

    column padding is 32 gap is 24:

        # Header
        row gap is 16:
            text "Dashboard" size is 28 bold color is dark navy
            spacer stretched
            button "New Report" background is indigo color is white rounded

        # Stat cards
        row gap is 16:
            card background is white rounded shadow:
                text "Total Users" size is 13 color is gray
                text "128,400" size is 32 bold color is dark navy
                text "↑ 12% this month" size is 13 color is green

            card background is white rounded shadow:
                text "Revenue" size is 13 color is gray
                text "$48,200" size is 32 bold color is dark navy
                text "↑ 8% this month" size is 13 color is green

            card background is white rounded shadow:
                text "Active Sessions" size is 13 color is gray
                text "3,920" size is 32 bold color is dark navy
                text "↓ 2% this month" size is 13 color is red

        # Main content
        row gap is 16:
            card background is white rounded shadow:
                text "Recent Activity" size is 16 bold color is dark navy
                divider
                text "User signup — Alice" size is 14 color is slate
                text "Payment received — $240" size is 14 color is slate
                text "Report exported — Q3" size is 14 color is slate

            card background is white rounded:
                text "Quick Actions" size is 16 bold color is dark navy
                divider
                button "Export Data" background is indigo color is white rounded stretched
                spacer 8
                button "Invite User" background is teal color is white rounded stretched
                spacer 8
                button "View Logs" background is white color is slate rounded stretched
```

**Total effort:** ~46 lines within 1 file.

This example highlights Rex's core philosophy: interfaces should read like design briefs, cleanly integrating visual structure and logic.
