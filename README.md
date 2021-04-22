![weeblang](https://user-images.githubusercontent.com/73722314/115655384-371eeb00-a301-11eb-85ec-ad9589d2d524.png)

Safely-typed scripting language written in Rust.
WeebLang itself has been inspired by both C++ and Rust but does not include the low-level functionality.
However, what the language lacks in low-level functionality, it makes up for it in its ease of use and readability.

Here is a code snippet to demonstrate syntax and semantics of WeebLang:
```
// All imports are initialized here
import {
  math
};

// WeebLang requires a main fucntion
fn main() {
  // for loops are formated like C-style for loops
  for (int i = 0; i <= 10; i++) {
    if isPrime(i) {
      print(i);
    }
  }
}

// Function declarations are similar to those in Rust
fn isPrime(input: int) -> (bool) {
  let is_prime: bool = TRUE;

  if input < 2 {
    is_prime = FALSE;
  } else {
    const limit: int = math::ceil(math::sqrt(input));
    for (int i = 2; i < limit; i++) {
      if input % i == 0 {
        is_prime = FALSE;
        break;
      }
    }
  }

  return is_prime;
}
```

## Roadmap
- [X] Port current code to Rust
- [ ] Create Parser
  - [ ] Create Lexer
  - [ ] Create Parse Tree
  - [ ] Configure Parser to fill Parse Tree
- [ ] Create code runner
  - [ ] create Parse Tree reader
  - [ ] create Executer
- [ ] Create WeebLang standard library
  - [ ] simple math
  - [ ] booleans
  - [ ] strings
- [ ] Create other libraries
  - [ ] Math library
  - [ ] Multiproccessing library
  - [ ] others TBD

## Contributing
If you wish to contribute contact me on Discord, integral#0400
