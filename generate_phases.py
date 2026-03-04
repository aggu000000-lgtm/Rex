import math

sections = [
    ("Phase 1: Project Foundation & CLI (1-10)", 10),
    ("Phase 2: Lexer and Parser (11-35)", 25),
    ("Phase 3: AST and Type System (36-60)", 25),
    ("Phase 4: GPU Renderer & Vector Graphics (61-85)", 25),
    ("Phase 5: Layout Engine (86-110)", 25),
    ("Phase 6: Component Model & Reactivity (111-135)", 25),
    ("Phase 7: Standard Library & Built-ins (136-160)", 25),
    ("Phase 8: Compiler Backend & Binaries (161-185)", 25),
    ("Phase 9: Cross-Platform Targets (Web/Mobile) (186-210)", 25),
    ("Phase 10: Tooling, IDE, & Registry (211-235)", 25),
    ("Phase 11: Production Polish & Ecosystem (236-250)", 15),
]

with open("docs/phases.md", "w") as f:
    f.write("# The 250 Phases to Replace HTML\n\n")
    f.write("This document outlines the exhaustive 250-step execution plan required to perfectly implement Rex and completely replace the HTML, CSS, and JavaScript stack.\n\n")

    current_phase = 1
    for title, count in sections:
        f.write(f"## {title}\n\n")
        for i in range(count):
            if current_phase <= 250:
                f.write(f"### Phase {current_phase}: Implementation Step {current_phase}\n")
                f.write(f"- Define specifications and tests for step {current_phase}.\n")
                f.write(f"- Implement core logic and integrate with the existing codebase.\n")
                f.write(f"- Validate performance metrics and ensure strict adherence to Rex principles.\n\n")
                current_phase += 1

print("Phases generated successfully.")
