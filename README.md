[![Contributors](https://img.shields.io/github/contributors/Builditluc/wiki-tui.svg?style=for-the-badge)](https://github.com/Builditluc/wiki-tui/graphs/contributors)
[![Stargazers](https://img.shields.io/github/stars/Builditluc/wiki-tui.svg?style=for-the-badge)](https://github.com/Builditluc/wiki-tui/stargazers)
[![Issues](https://img.shields.io/github/issues/Builditluc/wiki-tui.svg?style=for-the-badge)](https://github.com/Builditluc/wiki-tui/issues)
[![MIT license](https://img.shields.io/github/license/Builditluc/wiki-tui?style=for-the-badge)](https://github.com/Builditluc/wiki-tui/blob/stable/LICENSE.txt)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Builditluc/wiki-tui/Rust?style=for-the-badge)
![Lines of code](https://img.shields.io/tokei/lines/github/Builditluc/wiki-tui?style=for-the-badge)
![wakatime](https://wakatime.com/badge/github/Builditluc/wiki-tui.svg?style=for-the-badge)

<br />
<p align="center">
  <a href="https://github.com/Builditluc/wiki-tui">
    <img src= "logo.png" alt="Logo" width="234" height="167">
  </a>

  <h3 align="center">WIKI-TUI</h3>

  <p align="center">
    A simple and easy to use Wikipedia Text User Interface
  </p>
</p>

## Preview

### Features
`wiki-tui` currently has these features:
- Browse through Wikipedia (Set the language by changing the url in the config)
- Uses webscraping and a custom view to display wikipedia articles in the terminal


These features are planned:
- View and interact with more elements of wikipedia articles (like tables, images, etc.)
- Use links to open other articles

### How it looks

![image](https://user-images.githubusercontent.com/37375448/116461454-8ec0d780-a868-11eb-8725-a503bce4828c.png)
![image](https://user-images.githubusercontent.com/37375448/116461510-a0a27a80-a868-11eb-950b-f804ffa4ad3b.png)
![image](https://user-images.githubusercontent.com/37375448/116461593-bb74ef00-a868-11eb-9280-cf64eaa4e11f.png) <br>
Note: These Screenshots were taken on iTerm2 (MacOS) with the [spaceduck](https://github.com/pineapplegiant/spaceduck-terminal) theme and the SF Mono Font

## Installation

Currently, you can install `wiki-tui` only by compiling it manually.
Just clone the repository and compile the stable branch.
## Configuration

### Location of the config file
#### MacOS
```
$HOME/Library/Application Support/wiki-tui/config.ini
```
#### Linux
```
$HOME/.config/wiki-tui/config.ini
```
#### Windows
```
C:\Users\{USERNAME}\wiki-tui\config.ini
```

### Settings
#### Api
```ini
; this is the url of wikipedia, it can be changed to change the language of wikipedia 
BASE_URL = "https://en.wikipedia.org/"
```
#### Theme
The settings here are all colors and can be set by either the name of the color or a hex string (valid formats are: `#ffffff`, `#fff`). If your color wasn't applied, check the logs to find out why.
```ini
; color used for View backgrounds
background = white
; color used for the title text
title = red
; color used for highlighting text
highlight = red
; color used for highlighting inactive text
highlight_inactive = blue
; color used for highlighted text
highlight_text = white
; color used for the text
text = black
; color used for a search match in the results view
search_match = red
```
