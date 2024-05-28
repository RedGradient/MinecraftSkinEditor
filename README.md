![MCSkinEditor UI](resources/screenshot-1.png)

[//]: # (<p float="left">)
[//]: # (  <img src="resources/screenshot-1.png" width="400" /> )
[//]: # (</p>)

# Minecraft Skin Editor
> [!WARNING]
> ALPHA version

### How to run
```shell
$ git clone https://github.com/RedGradient/MinecraftSkinEditor.git
$ cd MinecraftSkinEditor
$ make build
```

### Known issues
* Artifacts on the model
* macOS: GTK warnings in console: "Broken accounting of active state for widget"
* 'Templates' and 'Reset' actions do not support undo/redo operations
* HTTP API request to skin API is synchronous
* Skins downloaded from the internet may fail to load because old skins with dimensions of 64x32 are not supported. The error about it will be displayed in console
* No asking to save before closing the App
