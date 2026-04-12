# Rex vs JavaScript: Core Logic & Type Safety

## JavaScript Idiosyncrasies and Tooling

JavaScript has evolved significantly from its inception as a lightweight scripting language into the foundation of complex web applications. However, it still carries historical burdens, including:

- **Type Coercion and Unpredictable Truthiness**: Expressions like `0 == "0"` evaluate to `true`, and `"0" == []` evaluates to `false`.
- **Complex "Object" Semantics**: Arrays, functions, and even `null` are treated as objects (`typeof null === "object"`).
- **Control Flow Evolution**: Transitioning from callbacks, to Promises, and finally `async/await`.

Due to these inconsistencies, JavaScript development heavily relies on supersets like TypeScript, linters (ESLint), and complex build pipelines (Webpack, Babel) for safe execution.

## The Rex Type System

Rex implements a strict, clear type system consisting of exactly five fundamental types. This ensures predictability and eliminates silent type coercion errors common in JavaScript.

| Type | Example |
|---|---|
| `text` | `"Hello, World!"` |
| `number` | `42`, `3.14`, `-7` |
| `bool` | `yes`, `no` |
| `list` | `[1, 2, 3]` |
| `nothing` | `nothing` |

Rex does not have `null`, `undefined`, or `NaN`. It does not support silent type coercion—attempting to add a number to text results in a compile-time error, preventing unexpected runtime states. Types are fully inferred by the compiler.

```rex
let name is "Alice"
let age is 30
let active is yes

# This will trigger a clear compile error in Rex
# let broken is name + age
```

## Variables and Reactivity

Traditional JavaScript requires DOM manipulation or a virtual DOM mechanism to reflect state changes.

**JavaScript:**
```javascript
let count = 0;
const increment = () => { count++; updateUI(); };
const decrement = () => { count--; updateUI(); };
function updateUI() {
  document.getElementById('count').innerText = count;
}
```

**Rex:**
Rex inherently understands state dependencies and automatically re-renders the UI when referenced variables change. No `updateUI()` function or `document.getElementById` lookup is required.

```rex
let count is 0

button "+":
    on click:
        count += 1

text "{count}" size is 48 bold centered

button "−":
    on click:
        count -= 1
```

## Conditionals and Loops

Rex implements highly readable control structures that forgo ternary operations, complicated nesting, and block syntax syntax (like curly braces or specific array methods).

### Conditionals

**JavaScript:**
```javascript
const message = count > 10
  ? count > 100
    ? "Enormous!"
    : "Getting big"
  : "Still small";
```

**Rex:**
```rex
if count > 100:
    text "Enormous!" color is red bold
else if count > 10:
    text "Getting big" color is orange
else:
    text "Still small" color is green
```

### Loops

**JavaScript:**
```javascript
const items = ["Apples", "Bananas", "Cherries"];
items.forEach(item => {
  const div = document.createElement('div');
  div.className = 'item-card';
  div.textContent = item;
  container.appendChild(div);
});
```

**Rex:**
```rex
let items is ["Apples", "Bananas", "Cherries"]

for item in items:
    card:
        text item size is 16
```

## Functions and Asynchronous Data Fetching

Rex provides a unified syntax for function definition and abstracts asynchronous operations natively.

### Functions

**JavaScript:**
```javascript
function greet(name) { return `Hello, ${name}!`; }
const greet = (name) => `Hello, ${name}!`;
const greet = (name) => { return `Hello, ${name}!`; };
```

**Rex:**
```rex
define greet(name):
    give back "Hello, {name}!"
```

### Async Operations

**JavaScript:**
```javascript
async function loadUser(id) {
  try {
    const res = await fetch(`https://api.example.com/users/${id}`);
    if (!res.ok) throw new Error('Failed');
    const data = await res.json();
    document.getElementById('username').textContent = data.name;
  } catch (err) {
    document.getElementById('error').textContent = err.message;
  }
}
```

**Rex:**
Rex abstracts Promise resolution into straightforward `on success` and `on error` blocks.

```rex
fetch user from "https://api.example.com/users/{id}":
    on success:
        text user.name size is 18 bold
    on error:
        text "Failed to load" color is red
```
