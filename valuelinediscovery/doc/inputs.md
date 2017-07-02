Handling Inputs
===============================

I want some scheme to deal with user inputs within the RTFM framework. I'll
want to handle at the minimum push-button switches and rotary encoders. And
things'll need to be responsive yet debounced.

First Idea
===============================
I'm thinking something like:

* Main code sets a timer with an interrupt
* Each time the interrupt fires, we read the inputs.

This task would be high priority, and the do the minimum to update states
of the things.
