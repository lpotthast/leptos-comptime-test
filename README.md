# leptos-comptime-test

Testing leptos application compile times with different scenarios for the `view!` macro.

## debug builds

Results of changing a string inside the critical view! macro in app.rs and re-running `trunk serve` (incremental debug build) or `trunk serve --release` after the app was already build before:

| test                                   | debug | release |
|----------------------------------------|-------|---------|
| 1_breadth                              | ~1.2s | ~0.5s   |
| 2_small_depth                          | ~1.4s | ~0.5s   |
| 3_big_depth                            | ~60s  | ~0.5s   |
| 3a_big_depth_leptos_0.3                | ~0.5s | ~0.5s   |
| 4_small_depth_with_breadth             | ~27s  | ~0.5s   |
| 4a_small_depth_with_breadth_leptos_0.3 | ~0.6s | ~0.5s   |
| 4b_small_depth_with_breadth_extracted  | ~1s   | ~0.5s   |
