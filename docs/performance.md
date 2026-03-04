# Rex Performance & Rendering

Rex bypasses standard browser rendering architectures to provide highly performant, visually consistent applications across all platforms.

## The Native GPU Renderer

Standard web frameworks rely on web browser layout engines and JavaScript runtimes (e.g., Chromium embedded in Electron). Rex, by contrast, renders UI directly to the GPU utilizing a retained-mode, vector graphics pipeline. This architectural decision enables a significant leap in visual quality and runtime performance.

### Rendering Characteristics

- **Subpixel Anti-Aliasing:** Rex renders shapes and text at the mathematical subpixel level matching your display resolution, unlike CSS which frequently rounds to whole pixels. The result is consistently crisp edges, perfect circles, and smooth rounded corners.
- **High Frame Rates:** Because it runs a native vector pipeline rather than a virtual DOM diff and repaint sequence, Rex operates at a baseline of 60fps, frequently pushing 120fps effortlessly. Animations, transitions, and hover effects incur negligible performance cost since they translate to direct GPU draw calls.
- **Cross-Platform Consistency:** A Rex application renders identically on Windows, macOS, and Linux. There are no engine inconsistencies between Chrome, Safari, or Firefox because Rex is the renderer.
- **Advanced Typography:** Rex includes a proper font rendering engine handling kerning, ligatures, and accurate text shaping natively, avoiding the discrepancies and hacks typical of web typography.

### Performance Profile

By eliminating engines like V8 and Chromium from the distribution, Rex applications start practically instantly and operate with a drastically smaller footprint.

| Metric | HTML + CSS + JS (Electron/Web) | Rex |
|---|---|---|
| App startup time | 2–8 seconds | < 50ms |
| Binary size | 120MB+ | < 5MB |
| RAM usage (hello world) | 200MB+ | < 15MB |
| Render engine | Browser (variable performance) | Dedicated GPU Vector Engine |
