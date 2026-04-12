# Rex Language Design

## Philosophy

Rex aims to be:
- **Simple** - English-like syntax anyone can read
- **Powerful** - Full web development capabilities
- **Clean** - No boilerplate, just code

## Syntax Overview

### Variables

```rex
make name = "John"
make age = 25
make isActive = true
```

### Output

```rex
show "Hello World"
show name
```

### Conditionals

```rex
if age > 18 {
    show "Adult"
}
```

### Loops

```rex
make i = 0
loop i < 10 {
    show i
    make i = i + 1
}
```

### Functions

```rex
func greet name {
    show "Hello " + name
    return "Done"
}
```

## Keywords

| Rex | JavaScript | HTML/CSS |
|-----|------------|----------|
| make | let/const | - |
| show | console.log() | - |
| if | if | - |
| loop | for/while | - |
| func | function | - |
| return | return | - |
| true | true | - |
| false | false | - |
| null | null | - |

## Roadmap

- [x] Lexer/Tokenizer
- [x] Parser
- [ ] AST improvements
- [ ] Code generation
- [ ] REPL
- [ ] Standard library
- [ ] Package manager