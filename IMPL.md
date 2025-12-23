# Implementation Strategy

In personal projects, it is more important to complete something than to make it perfect. To that end, I'm tracking the following strategy:

- complete the engine first: I'll build a funcitonal rummy framework that supports a basic gameplay api. I'll focus on providing the ability to make moves over analysis of optimal moves.
- Aim for realism, but fallback to convenience: I'd love this system to support all edge cases and intricacies of real play, such as the variable player numbers, last-hand special rules, and whatnot. But a prudent approach suggests writing this up for the simple base case first
- ignore optimization initially: i'm gonna get a complete working solution first before I worry AT ALL about performance optimization. It hurts, but it works