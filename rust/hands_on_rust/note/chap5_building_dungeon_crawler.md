# Building A Dungeon Crawler

- Project Name: Rusty Roguelike
- Short Description:
  A dungeon crawler with procedurally generated levels, monsters of increasing difficulty, and turn-based movement.
- Story
- Basic Game Loop
  - Enter dungeon level
  - Explore, revealing the map
  - Encounter enemies whom the player fights or flees from.
  - Find power-ups and use them to strengthen the player
  - Locate the exit to the level - go to 1
- Minimum Viable Product
  - Create a basic dungeon map
  - Place the player and let them walk around
  - Spawn monsters, draw them, and let the player kill them by walking into them.
  - Add health and a combat system that uses it.
  - Add healing potions
  - Display a "game over" screen when the player dies.
  - Add the Amulet of Yala to the level and let the player win by reaching it.
- Stretch Goals
  - Add Fields-of-View
  - Add more interesting dungeon designs
  - Add some dungeon themes
  - Add multiple levels of dungeon, with the Amulet on the last one,
  - Add varied weapons to the game
  - Move to a data-driven design for spawning enemies
  - Consider some visual effects to make combat more visceral
  - Consider keeping score


## Dividing Your Code Into Modules
- Easy to find the code
- Cargo can compile modules concurrently, leading to much better compilation performance
- Bugs are easier to find in self-contained code

### Crates and Modules

- `Crates`: Large groups of code with their own `Cargo.toml` file.

Crates and modules act as `namespace`, `bracket-lib::prelude` refers to the `bracket-lib` crate's `prelude` module.
Use can reference the current crate with `crate::`, for example, `map` module could be referred with `crate::map` in the same crate.

### Make a Stub Map Module

```yaml
# dependency on bracket-lib, newer than 0.8.1
[dependencies]
bracket-lib = "~0.8.1"
```

The below code imports `map` module into the global scope and sets up the `map::` prefix.
```rust
// in main.rs
mod map;
map::my_func;

// or
use map::my_func;
my_func;
```

### Module Scoping

- Modules are self-contained and have their own scope
- By default, everything in the module is private to the module.
- Making module entries `public` with the `pub` keyword:
  - Functions (e.g. `pub fn my_function()`)
  - Structs (e.g. `pub struct MyStruct`)
  - Enumerations (e.g. `pub enum MyEnum`)
  - Implemented functions (e.g. `impl MyStruct { pub fn my_function() }`)


## Organizing Your Imports With a Prelude
Prefixing every map access with `map::` or `crate::map::` is unwieldy and will get more cumbersome as you add more modules.
It's common for the library author to have placed everything you need in a convenient `prelude`.

```rust
// add the module to project with mod
mod map;


mod prelude {
// Publicly using the bracket_lib prelude re-exports it inside your prelude.
// pub to makes them public
  pub use bracket_lib::prelude::*;
// Make the declaring available to any section of code that uses your prelude.
  pub const SCREEN_WIDTH: i32 = 80;
  pub const SCREEN_HEIGHT: i32 = 50;
// Re-export the map as a public module available from within your prelude.
  pub use create::map::*;
}

use prelude::*;
```

### Set Up Your Map Module

```rust
use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
```

## Storing the Dungeon Map
Most two-dimensional games represent their map as a series of tiles in a grid pattern. Each tile has a type describing how that tile is rendered—and what happens if you try to enter it.

### Represent Your Tiles
Tiles are limited to a pre-defined set of tile types, making them perfect for an enum.

```rust
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
  Wall,
  Floor,
}
```

- `Copy`
  `Copy` changes the default action then assigning a `TileType` from one variable to another.
  Instead of moving the value, it makes a copy. Clippy will warn you if you are borrowing a variable and it would be faster to copy it.

- `Clone`
  `Clone` adds a clone function to the type, Calling `mytile.clone` makes a deep copy of the variable without affecting the original.

- `PartialEq`
  `PartialEq` adds code that allows you to compare `TileType` with "==" operator.

### Create an Empty Map

```rust
pub struct Map {
  pub tiles: Vec<TileType>,
}

impl Map {
  // construction function
  pub fn new() -> Self {
    Self {
      tiles: vec![TileType::Floor; NUM_TILES],
    }
  }
}
```

### Index the Map
Vectors are indexed on a single dimension, you need to transform map location (x,y) into vector indices. The transformation is known as `striding`.

```rust
pub fn map_idx(x: i32, y: i32) -> usize {
  ((y * SCREEN_WIDTH) + x) as usize
}

x = idx % SCREEN_WIDTH;
y = idx / SCREEN_WIDTH;
```

### Render the Map
```rust
  pub fn render(&self, ctx: &mut BTerm) {
    for y in 0..SCREEN_HEIGHT {
      for x in 0..SCREEN_WIDTH {
        let idx = map_idx(x, y);
         match self.tiles[idx] {
           TileType::Wall => {
             ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
           }
           TileType::Floor => {
            ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
           }
         }
      }
    }
  }
```

### Consume the Map API

