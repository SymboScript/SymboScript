<div style="text-align: center;">
      <h1>SymboScript</h1>
      <a href="https://crates.io/crates/symboscript-interpreter">
            <img src="https://img.shields.io/crates/v/symboscript-interpreter?label=interpreter" alt="Crates.io Version interpreter">
      </a>
      <a href="https://crates.io/crates/symboscript-optimizer">
            <img src="https://img.shields.io/crates/v/symboscript-optimizer?label=optimizer" alt="Crates.io Version optimizer">
      </a>
      <a href="https://crates.io/crates/symboscript-formatter">
            <img src="https://img.shields.io/crates/v/symboscript-formatter?label=formatter" alt="Crates.io Version formatter">
      </a>
      <a href="https://crates.io/crates/symboscript-parser">
            <img src="https://img.shields.io/crates/v/symboscript-parser?label=parser" alt="Crates.io Version parser">
      </a>
      <a href="https://crates.io/crates/symboscript-lexer">
            <img src="https://img.shields.io/crates/v/symboscript-lexer?label=lexer" alt="Crates.io Version lexer">
      </a>
      <a href="https://crates.io/crates/symboscript-utils">
            <img src="https://img.shields.io/crates/v/symboscript-utils?label=utils" alt="Crates.io Version utils">
      </a>
      <a href="https://crates.io/crates/symboscript-types">
            <img src="https://img.shields.io/crates/v/symboscript-types?label=types" alt="Crates.io Version types">
      </a>
      <p>SymboScript is a a language without limits.</p>
</div>

## Idea of SymboScript

Take JavaScript, Rust, and WolframLanguage and make a great language out of them without any restrictions.

## Hello, World!

Check the interpreter crate - [Interpreter](./interpreter/readme.md)
Check the examples - [Examples](./examples/)

```syms
println["Hello, World!"];
```

## Roadmap (MVP)

- [ ] [Documentation](https://symboscript.github.io/Book/) (started)
- [x] [Lexer](./lexer/readme.md) (started)
- [x] [Parser](./parser/readme.md) (started)
- [ ] [Optimizer](./optimizer/readme.md) (started, not really)
- [ ] [Interpreter](./interpreter/readme.md) (started)
- [x] REPL in [Interpreter](./interpreter/readme.md)
- [ ] Analyzer
- [ ] [Format](./formatter/readme.md) (started, not really)
- [ ] Package manager
