The `map` function on `Option` takes an `Option<T>` and a closure `f` (which implements `FnOnce`) and applies the closure to the value inside the `Option` if it's `Some(T)`. If the `Option` is `None`, it just returns `None` without applying the closure.

Here's a simplified version of how `map` works:

```rust
pub fn map<F, T, A>(option: Option<T>, f: F) -> Option<A>
where
    F: FnOnce(T) -> A,
{
    match option {
        Some(value) => Some(f(value)),
        None => None,
    }
}
```

- `F: FnOnce(T) -> A` means that the closure `f` takes an argument of type `T` and returns a value of type `A`.
- If `option` is `Some(T)`, `map` applies the closure `f` to `T` and returns `Some(A)`.
- If `option` is `None`, it returns `None`.

### Example:

```rust
let opt = Some(5);
let result = opt.map(|x| x * 2);  // Closure doubles the value

assert_eq!(result, Some(10));
```

If the `Option` is `Some(5)`, `map` applies the closure `|x| x * 2`, resulting in `Some(10)`. If the `Option` was `None`, the result would also be `None`.

### `Fn`, `FnMut`, and `FnOnce`:

- **`FnOnce`**: The closure can be called **once** and might consume the variables it captures. This is because it might take ownership of captured variables.
  
  ```rust
  let s = String::from("hello");
  let closure = || s; // Moves `s` into the closure
  println!("{}", closure()); // Closure consumes `s`
  // println!("{}", s); // Error: `s` was moved
  ```

- **`FnMut`**: The closure can modify the variables it captures. It requires **mutable** access to captured variables.
  
  ```rust
  let mut x = 5;
  let mut closure = || x += 1;
  closure(); // Modifies `x`
  println!("{}", x); // x is now 6
  ```

- **`Fn`**: The closure can be called multiple times without mutating or consuming the captured variables. It requires **immutable** access to captured variables.
  
  ```rust
  let x = 5;
  let closure = || x; // Captures `x` by reference
  println!("{}", closure()); // Outputs 5
  ```

### Summary:

- `FnOnce`: Consumes captured variables, can be called once.
- `FnMut`: Can mutate captured variables, can be called multiple times.
- `Fn`: Immutable, can be called multiple times.
