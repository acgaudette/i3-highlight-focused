# i3-highlight-focused

A background process that temporarily highlights the active window in i3 as focus changes. The border width and delay (in ms) must be passed as arguments.

## Installation

`cargo build`

## Usage

`i3-highlight-focused width delay`

### In your i3 config

```
exec_always --no-startup-id \
  i3-highlight-focused 8 256
```
