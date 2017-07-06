Liquid Crystal Display
=============================

This works now kinda. The `example/button_and_lcd.rs` needs a few tweaks,
namely:

1. Fix timing so that it's the same (or at least works) in both `release`
  and debug modes.
2. The switch polling rate needs to be ~400Hz for it to feel responsive
  to me in debug mode.
2. Fix the display update, currently it's updating in each `loop` and
  is causing flickering on the display.


