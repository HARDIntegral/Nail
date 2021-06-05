![Nail (2)](https://user-images.githubusercontent.com/73722314/116011990-d68bf880-a5f5-11eb-8aca-0064b93ba2ac.png)

Safely-typed scripting language written in Rust.
Nail itself has been modeled like Typescript and inspired by Rust but does not include the low-level functionality.
However, what the language lacks in low-level functionality, it makes up for it in its ease of use and readability.

Here is a code snippet to demonstrate syntax and semantics of Nail:
```
sys.print("Hello Nail!")

for i in range 0..10 {
    sys.print("Hehe Loops")
}
```

## Roadmap
- [X] Port current code to Rust
- [ ] Create Parser
  - [ ] Create Lexer
  - [X] Create Parse Tree
  - [ ] Configure Parser to fill Parse Tree
  - [ ] create Parse Tree reader
  - [ ] create Executer
- [ ] Create Nail standard library
  - [ ] simple math
  - [ ] booleans
  - [ ] strings
  - [ ] Data Structures
    - [ ] Lists
    - [ ] Linked Lists
    - [ ] Stacks
    - [ ] Queues
    - [ ] Trees
- [ ] Create other libraries
  - [ ] Math library
  - [ ] Multiproccessing library
  - [ ] others TBD

## Contributing
If you wish to contribute contact me on Discord, integral#0400
