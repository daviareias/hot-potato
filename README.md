# Hot Potato
A simple preview tool with hot reloading for HTML and tailwind

**This just an experimental project**
## Description
Hot potato is a simple tool for people who want to preview their HTML files
# Getting Started
## Installation
Hot Potato watches for changes in any file inside the "ui" directory, it them automatically reloads any page that is opened in your browser and has an active websocket connection.

In the future you'll be able to use any folder.

## Tauri
You can also use Hot potato as a substitute for nodeJS when previewing changes in Tauri, just make sure to open tauri.conf.json and change the following values to your prefered values:
```
"devPath": "../ui",
"distDir": "http://localhost:8080/index.html",
```
