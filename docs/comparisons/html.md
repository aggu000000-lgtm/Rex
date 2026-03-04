# Rex vs HTML: UI Structure

Traditional web development often separates structure (HTML) and style (CSS) across multiple files. This separation often necessitates extensive boilerplate, complex class hierarchies, and non-intuitive layout behaviors (e.g., centering items, which requires multiple CSS properties).

Rex unifies these concerns into a single, declarative syntax. It does away with angle brackets, semantic document tags, and explicit classes or IDs. Properties are applied immediately to the elements they modify.

## Examples

### 1. Centering Content

**In HTML + CSS:**
```html
<div style="display: flex; justify-content: center; align-items: center; height: 100vh;">
  <div class="card">
    <h1 class="title">Hello</h1>
  </div>
</div>
```
```css
.card {
  background: white;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 4px 24px rgba(0,0,0,0.08);
}
.title {
  font-size: 32px;
  font-weight: 700;
  color: #1e1b4b;
}
```

This traditional approach requires approximately 16 lines spread across two contexts.

**In Rex:**
```rex
column centered padding is 40:
    card:
        text "Hello" size is 32 bold color is dark indigo
```

This is defined concisely in 3 lines within a single file, eliminating classes and stylesheets.

---

### 2. A Navigation Bar

**In HTML + CSS:**
```html
<nav class="navbar">
  <div class="navbar-brand">MyApp</div>
  <ul class="navbar-links">
    <li><a href="/home" class="nav-link active">Home</a></li>
    <li><a href="/about" class="nav-link">About</a></li>
    <li><a href="/contact" class="nav-link">Contact</a></li>
  </ul>
  <button class="cta-button">Get Started</button>
</nav>
```
```css
.navbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 32px;
  background: white;
  border-bottom: 1px solid #e2e8f0;
  box-shadow: 0 1px 4px rgba(0,0,0,0.06);
}
.navbar-brand {
  font-size: 20px;
  font-weight: 800;
  color: #4f46e5;
}
.navbar-links {
  display: flex;
  list-style: none;
  gap: 24px;
}
.nav-link {
  text-decoration: none;
  color: #64748b;
  font-size: 15px;
}
.nav-link.active {
  color: #4f46e5;
  font-weight: 600;
}
.cta-button {
  background: #4f46e5;
  color: white;
  border: none;
  border-radius: 8px;
  padding: 10px 20px;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
}
```

**In Rex:**
```rex
row padding is 16 gap is 24 background is white shadow:
    text "MyApp" size is 20 bold color is indigo
    text "Home" size is 15 bold color is indigo
    text "About" size is 15 color is slate
    text "Contact" size is 15 color is slate
    button "Get Started" background is indigo color is white rounded
```

The definition of the navbar is inherently readable and contained in 5 lines.

---

### 3. A Signup Form

**In HTML + CSS + JavaScript:**
```html
<form id="signupForm" class="form-container">
  <h2 class="form-title">Create Account</h2>
  <div class="form-group">
    <label for="name">Full Name</label>
    <input type="text" id="name" class="form-input" placeholder="John Doe">
  </div>
  <div class="form-group">
    <label for="email">Email Address</label>
    <input type="email" id="email" class="form-input" placeholder="john@example.com">
  </div>
  <div class="form-group">
    <label for="password">Password</label>
    <input type="password" id="password" class="form-input" placeholder="••••••••">
  </div>
  <button type="submit" class="submit-btn">Create Account</button>
</form>
```
*(Plus associated CSS and JS logic handling submission and validation)*

**In Rex:**
```rex
card padding is 32 rounded shadow:
    text "Create Account" size is 24 bold color is dark navy
    spacer 8
    text "Full Name" size is 14 color is slate
    input "John Doe" width is 340
    text "Email Address" size is 14 color is slate
    input "john@example.com" width is 340
    text "Password" size is 14 color is slate
    input "••••••••" width is 340
    spacer 8
    button "Create Account":
        background is indigo
        color is white
        rounded
        stretched
        on click:
            show "Account created! 🎉"
```

The 18-line Rex snippet completely captures layout, styling, elements, and dynamic logic.
