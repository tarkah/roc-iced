module [
    Row,
    row,
    spacing,
    padding,
    width,
    height,
    alignItems,
    clip,
]

import Alignment exposing [Alignment]
import Length exposing [Length]
import Padding exposing [Padding]

Row elem : {
    children : List elem,
    spacing : F32,
    padding : Padding,
    width : Length,
    height : Length,
    alignItems : Alignment,
    clip : Bool,
}

row = \children ->
    Row {
        children,
        spacing: 0,
        padding: Padding.zero,
        width: Shrink,
        height: Shrink,
        maxWidth: Num.infinityF32,
        alignItems: Start,
        clip: Bool.false,
    }

spacing = \elem, s -> update elem \c -> { c & spacing: s }

padding = \elem, p -> update elem \c -> { c & padding: Padding.padding p }

width = \elem, w -> update elem \c -> { c & width: Some w }

height = \elem, h -> update elem \c -> { c & height: Some h }

alignItems = \elem, a -> update elem \c -> { c & alignItems: a }

clip = \elem, b -> update elem \c -> { c & clip: b }

update = \elem, f ->
    when elem is
        Row c -> Row (f c)
        _ -> elem

