# Reproduction of `winit` but affecting Alacritty

## Alacritty issue

See [this comment]() for details.


## TLDR

Current implementation of main loop in `winit` on Windows required all the
Windows messages to be handled before `ControlFlow::Exit` status is checked
and main loop terminates.

In a scenario, where event handler is slow and another thread in the application
overproduced events, the application can be stuck forever handling the events
and never terminate, even when `ControlFlow::Exit` is set.
