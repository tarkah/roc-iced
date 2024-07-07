module [
    Settings,
    Window,
    default,
    antialiasing,
    defaultTextSize,
    size,
    minSize,
    maxSize,
    resizable,
    decorations,
    transparent,
]

import Option exposing [Option]
import Size exposing [Size]

Settings : {
    antialiasing : Bool,
    defaultTextSize : F32,
    window : Window,
}

Window : {
    size : Size,
    minSize : Option Size,
    maxSize : Option Size,
    resizable : Bool,
    decorations : Bool,
    transparent : Bool,
}

default : Settings
default = {
    antialiasing: Bool.false,
    defaultTextSize: 16,
    window: {
        size: {
            width: 1024,
            height: 768,
        },
        minSize: None,
        maxSize: None,
        resizable: Bool.true,
        decorations: Bool.true,
        transparent: Bool.false,
    },
}

antialiasing = \a, b -> { a & antialiasing: b }
defaultTextSize = \a, b -> { a & defaultTextSize: b }
size = \a, b -> a.window |> \w -> { a & window: { w & size: b } }
minSize = \a, b -> a.window |> \w -> { a & window: { w & minSize: Some b } }
maxSize = \a, b -> a.window |> \w -> { a & window: { w & maxSize: Some b } }
resizable = \a, b -> a.window |> \w -> { a & window: { w & resizable: b } }
decorations = \a, b -> a.window |> \w -> { a & window: { w & decorations: b } }
transparent = \a, b -> a.window |> \w -> { a & window: { w & transparent: b } }

