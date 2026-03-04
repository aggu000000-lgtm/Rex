# Introduction to Rex

Rex is a next-generation native UI programming language implemented in Rust. Rather than acting as a framework, library, or wrapper over existing web technologies, Rex provides a comprehensive programming language encompassing its own syntax, compiler, renderer, and runtime environment. Its primary objective is to offer a streamlined, highly performant alternative to the traditional HTML, CSS, and JavaScript stack.

## Core Characteristics

- **Declarative, Readable Syntax**: Rex is designed to be highly readable, eschewing standard web constructs such as angle brackets, semicolons, and explicit class-based styling. Developers declare UI components in a declarative format that closely resembles plain English, allowing for concise definitions of layouts and behavior.
- **Native GPU Rendering**: Rex bypasses web browser layout engines entirely, rendering directly to the GPU using a custom vector graphics pipeline. This eliminates the need for a virtual DOM, runtime environments like Node.js, or large JavaScript bundles.
- **Compiled Binaries**: Applications written in Rex compile into lean native binaries that launch instantly and execute efficiently with high frame rates (e.g., 120fps).

## Motivation

The conventional web stack relies heavily on HTML (originally a document formatting language), CSS (a stylesheet language), and JavaScript. While highly flexible, combining these technologies often introduces significant overhead, layout complexity, and runtime errors. Rex provides a clean break from these paradigms. It is designed from inception strictly for building native interfaces, unifying structure, styling, and logic into a single cohesive language.

Rex is written in Rust, leveraging its performance and safety guarantees to deliver a rapid, predictable, and visually consistent UI development experience.
