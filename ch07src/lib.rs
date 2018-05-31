pub mod client;

pub mod network;

#[cfg(test)]
mod tests;
/* Logical module hierarchy
communicator    (root of library: lib.rs)
 ├── client     (simple / single-file module: client.rs)
 └── network    (complex / folder module: network/mod.rs)
     └── server (simple / submodule: network/server.rs)

Module file rules (recursive):
- If a module named foo has no submodules, you should put the declarations for foo in a file named foo.rs.
- If a module named foo does have submodules, you should put the declarations for foo in a file named foo/mod.rs.

Visibility rules:
1. If an item is public, it can be accessed through any of its parent modules.
2. If an item is private, it can be accessed only by its immediate parent module and any of the parent’s child modules.
*/

// // no difference being pub from `try_me`
// mod outermost {
//     pub fn middle_function() {}

//     fn middle_secret_function() {}

//     // no difference being pub from `try_me` because children are private
//     mod inside {
//         fn inner_function() {
//             // Should be unproblematic because (2)
//             ::outermost::middle_secret_function();
//         }

//         fn secret_function() {}
//     }
// }

// fn try_me() {
//     // Compiles because (1)
//     outermost::middle_function();
//     // Does not compile because (2) and try_me is outside
//     outermost::middle_secret_function();
//     // Does not compile because `inside` is not pub
//     outermost::inside::inner_function();
//     // Does not compile because `inside` is not pub
//     outermost::inside::secret_function();
// }