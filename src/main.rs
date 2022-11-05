// this defines a module, sets up the map:: prefix
mod map; // this adds the map module to the project.

// modules are self contained and have their own scope.
// by default everything inside this module is private to this module.
// module entries can be made public with the pub keyword.
// its considered good design to keep majority of a module private
// and only publish a carefully designed set of functions and structures.
// modules should be deep, they should have a simple interface
// to access lots of functionality. A deep module hides complexity.
// General purpose modules are deeper than single purpose modules.

// prefixing every map access with map:: or crate::map:: is unwieldy
// and will get more cumbersome as more modules are added.
// when accessing a Rust library, its common for the author to place
// everything needed in a prelude.

// this declares a new prelude module it is automatically public
// as it is at the top level of the crate.
mod prelude {
    // publically using bracket-lib prelude re-exports it inside the map prelude.
    // this means that anything that uses the map prelude, also uses bracket-lib
    pub use bracket_lib::prelude::*;
    // adding pub to constants, makes them public.
    // declaring them in the map prelude makes them available
    // to any code that uses the map prelude.
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    // The map module was imported to main scope.
    // The Map module can therefore reference the main scope as crate::
    // This re-exports the map module as a public module available within the prelude.
    pub use crate::map::*;
}

// using the prelude makes it available to the main scope in main.rs
use prelude::*;
// before the prelude was written this file would
// use bracket_lib::prelude::*;

// any module that now uses the prelude has access
// to bracket-lib and all of the map module.

// modules are organised as a tree. When imports access other modules with use,
// there are several ways of accessing the module tree.
// super:: accesses the module immediately above that module in the tree
// crate:: accesses the root of the tree, main.rs

struct State {
    map: Map,
}
impl State {
    fn new() -> Self {
        Self { map: Map::new() }
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx)
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0) // automatically track game speed
        .build()?;

    main_loop(context, State::new())
}
