![Nail (2)](https://user-images.githubusercontent.com/73722314/116011990-d68bf880-a5f5-11eb-8aca-0064b93ba2ac.png)

Safely-typed scripting language written in Rust.
Nail itself has been inspired by Rust but does not include the low-level functionality.
However, what the language lacks in low-level functionality, it makes up for it in its ease of use and readability.

Here is a code snippet to demonstrate syntax and semantics of Nail:
```
// All imports are initialized here and specific
// methods can be defined here
import {
  math,
  math::sqrt as root,
}

fn isPrime(input: int) -> (bool) {
  let is_prime : bool = TRUE;

  if input < 2 {
    is_prime = FALSE;
  } else {
    const limit : int = math::ceil(root(input));
    for 2..limit {
      if input % i == 0 {
        is_prime = FALSE;
        break;
      }
    }
  }

  return is_prime;
}

// Nail requires a main fucntion
fn main() {
  for i in 0..10 {
    if isPrime(i) {
      print(i);
    }
  }
}
```

## Roadmap
- [X] Port current code to Rust
- [ ] Create Parser
  - [ ] Create Lexer
  - [X] Create Parse Tree
  - [ ] Configure Parser to fill Parse Tree
- [ ] Create code runner
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
