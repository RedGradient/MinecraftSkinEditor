![MCSkinEditor UI](resources/screenshot-1.png)

[//]: # (<p float="left">)
[//]: # (  <img src="resources/screenshot-1.png" width="400" /> )
[//]: # (</p>)

# Minecraft Skin Editor
> ⚠️ **ALPHA version**

### How to run
```shell
$ git clone https://github.com/RedGradient/MinecraftSkinEditor.git
$ cd MinecraftSkinEditor
$ make build
```

### Known issues
* Linux: wrong calculation of click on the GLArea
* Artifacts on the model
* macOS: GTK warnings in console: "Broken accounting of active state for widget"
* 'Wardrobe' and 'Reset' actions do not support undo/redo operations
* HTTP API request to skin API is synchronous
* No asking to save before closing the App
