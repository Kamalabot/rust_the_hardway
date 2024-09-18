In Rust, you can combine `match` arms with `if` conditions (known as *match guards*) to add additional conditions for specific arms. This allows you to match on a value while also adding logic to further filter which branch of the `match` is executed based on an `if` statement.

### Syntax Example:

```rust
match value {
    pattern if condition => { 
        // handle the case where `pattern` matches and `condition` is true 
    },
    other_pattern => {
        // handle the default or other patterns
    },
}
```

The `if condition` acts as an additional filter that applies only if the `pattern` matches.

### Example 1: Matching Numbers with Conditions

```rust
fn describe_number(n: i32) -> &'static str {
    match n {
        x if x > 0 => "positive",
        x if x < 0 => "negative",
        _ => "zero", // fallback case
    }
}

fn main() {
    println!("{}", describe_number(10));  // Output: positive
    println!("{}", describe_number(-5));  // Output: negative
    println!("{}", describe_number(0));   // Output: zero
}
```

In this example:

- The first arm matches when `n` is positive (`x > 0`).
- The second arm matches when `n` is negative (`x < 0`).
- The final arm (`_`) is the fallback for when `n == 0`.

### Example 2: Matching with Tuples and Conditions

You can also use match guards with more complex data, such as tuples:

```rust
fn describe_point(x: i32, y: i32) -> &'static str {
    match (x, y) {
        (0, 0) => "at the origin",
        (x, y) if x == y => "on the diagonal",
        (x, _) if x > 0 => "on the right half-plane",
        _ => "somewhere else",
    }
}

fn main() {
    println!("{}", describe_point(0, 0));   // Output: at the origin
    println!("{}", describe_point(5, 5));   // Output: on the diagonal
    println!("{}", describe_point(3, 2));   // Output: on the right half-plane
    println!("{}", describe_point(-3, 2));  // Output: somewhere else
}
```

- `(0, 0)` matches the origin.
- The second arm matches points where `x == y` (the diagonal).
- The third arm uses `x > 0` to match points in the right half-plane.
- The `_` arm handles all other cases.

### Example 3: Matching Enums with Conditions

You can also use match guards with enums. For instance:

```rust
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn describe_temperature(temp: Temperature) -> &'static str {
    match temp {
        Temperature::Celsius(t) if t > 30 => "It's hot in Celsius",
        Temperature::Fahrenheit(t) if t > 86 => "It's hot in Fahrenheit",
        Temperature::Celsius(t) if t <= 0 => "It's freezing in Celsius",
        Temperature::Fahrenheit(t) if t <= 32 => "It's freezing in Fahrenheit",
        _ => "It's a moderate temperature",
    }
}

fn main() {
    println!("{}", describe_temperature(Temperature::Celsius(35)));   // Output: It's hot in Celsius
    println!("{}", describe_temperature(Temperature::Fahrenheit(90))); // Output: It's hot in Fahrenheit
    println!("{}", describe_temperature(Temperature::Celsius(-5)));   // Output: It's freezing in Celsius
}
```

In this case:

- Each arm matches a variant of the `Temperature` enum.
- The `if` conditions filter the values to determine if it's "hot" or "freezing" for each temperature scale.

### Conclusion

Match guards (using `if` conditions inside `match` arms) allow you to add additional logic to pattern matching. They provide a flexible way to filter values based on conditions, and they're commonly used when matching complex types or patterns that require further checks beyond just the structure of the data.

In Rust, `match` expressions can be used in simple forms for basic pattern matching, but they can also handle complex cases by incorporating guards, nested patterns, or multiple conditions. Let's explore this progression from simple to complex match usage with a single use case: handling different states in a traffic light system.

### Use Case: Traffic Light System

We'll consider a `TrafficLight` enum representing three states:

1. `Red`
2. `Yellow`
3. `Green`

We will progressively add complexity to the `match` expression.

### 1. **Simple Match**

The most basic form of a `match` simply matches on different values of the enum.

#### Enum Definition:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
```

#### Simple Match Example:

```rust
fn light_action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow => "Slow down",
        TrafficLight::Green => "Go",
    }
}

fn main() {
    let light = TrafficLight::Red;
    println!("{}", light_action(light)); // Output: Stop
}
```

In this example:

- Each variant of the `TrafficLight` enum is matched to a specific action.

### 2. **Match with Additional Conditions (Guards)**

You can add conditions (guards) using `if` to refine how `match` works. For example, you can have a condition for the `Yellow` light based on whether you are close to the intersection or not.

#### Enum Definition with Time Variant:

```rust
enum TrafficLight {
    Red,
    Yellow(i32), // Time remaining for yellow light in seconds
    Green,
}
```

#### Match with Guards Example:

```rust
fn light_action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow(time) if time > 3 => "Slow down safely",
        TrafficLight::Yellow(_) => "Prepare to stop",
        TrafficLight::Green => "Go",
    }
}

fn main() {
    let light = TrafficLight::Yellow(4);
    println!("{}", light_action(light)); // Output: Slow down safely
}
```

In this example:

- **Match Guard**: For `TrafficLight::Yellow`, the match guard `if time > 3` provides an additional condition based on the time left for the yellow light.
- If the time left is greater than 3 seconds, the action is to "Slow down safely." Otherwise, the action is "Prepare to stop."

### 3. **Nested Pattern Matching**

You can match more complex types, such as when a struct or enum contains other enums or data.

#### Enum with Nested Data Example:

```rust
struct LightStatus {
    light: TrafficLight,
    emergency: bool,
}

enum TrafficLight {
    Red,
    Yellow(i32), // Time remaining for yellow light
    Green,
}
```

#### Nested Match Example:

```rust
fn light_action(status: LightStatus) -> &'static str {
    match status {
        LightStatus { light: TrafficLight::Red, emergency: true } => "Stop but give way",
        LightStatus { light: TrafficLight::Red, .. } => "Stop",
        LightStatus { light: TrafficLight::Yellow(time), .. } if time > 3 => "Slow down safely",
        LightStatus { light: TrafficLight::Yellow(_), .. } => "Prepare to stop",
        LightStatus { light: TrafficLight::Green, emergency: true } => "Go carefully",
        LightStatus { light: TrafficLight::Green, .. } => "Go",
    }
}

fn main() {
    let status = LightStatus {
        light: TrafficLight::Green,
        emergency: true,
    };
    println!("{}", light_action(status)); // Output: Go carefully
}
```

In this example:

- **Nested Pattern Matching**: The `LightStatus` struct contains both a `TrafficLight` value and an `emergency` boolean. We match on both of these simultaneously.
- **Match Guard**: In addition to matching the `Yellow` light, we also check the remaining time with `if time > 3`.

### 4. **Complex Enum Matching with Multiple Conditions**

You can match multiple conditions and even decompose multiple enums simultaneously.

#### Multiple Conditions Example:

```rust
struct LightStatus {
    light: TrafficLight,
    time_of_day: TimeOfDay, // A new enum to represent day or night
}

enum TrafficLight {
    Red,
    Yellow(i32),
    Green,
}

enum TimeOfDay {
    Day,
    Night,
}
```

#### Complex Match Example:

```rust
fn light_action(status: LightStatus) -> &'static str {
    match status {
        // Emergency case: Red light during night, give way to emergency vehicles
        LightStatus { light: TrafficLight::Red, time_of_day: TimeOfDay::Night } => "Stop but be cautious for emergency",
        // Red light during the day or other times
        LightStatus { light: TrafficLight::Red, .. } => "Stop",
        // Yellow light with different actions based on time of day
        LightStatus { light: TrafficLight::Yellow(time), time_of_day: TimeOfDay::Day } if time > 3 => "Slow down safely (Day)",
        LightStatus { light: TrafficLight::Yellow(_), time_of_day: TimeOfDay::Day } => "Prepare to stop (Day)",
        LightStatus { light: TrafficLight::Yellow(_), time_of_day: TimeOfDay::Night } => "Prepare to stop (Night)",
        // Green light at any time
        LightStatus { light: TrafficLight::Green, .. } => "Go",
    }
}

fn main() {
    let status = LightStatus {
        light: TrafficLight::Yellow(2),
        time_of_day: TimeOfDay::Night,
    };
    println!("{}", light_action(status)); // Output: Prepare to stop (Night)
}
```

In this example:

- We match not only on the `TrafficLight` but also on `TimeOfDay` to decide the action.
- **Multiple conditions**: Different actions are taken based on the time of day and the remaining time for the yellow light.

### Summary

1. **Simple Match**: Basic matching on enum variants.
2. **Match with Guards**: Adding conditions (`if`) to refine how matching works.
3. **Nested Pattern Matching**: Matching on structs or enums with more complex data.
4. **Multiple Conditions**: Combining multiple match conditions and enums for fine-grained control.

This progression demonstrates how `match` in Rust can start simply and grow in complexity, handling increasingly nuanced situations in real-world code.
