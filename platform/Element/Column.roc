module [
    Column,
    column,
    spacing,
    padding,
    width,
    height,
    maxWidth,
    alignItems,
    clip,
]

import Alignment exposing [Alignment]
import Length exposing [Length]
import Padding exposing [Padding]

Column elem : {
    children : List elem,
    spacing : F32,
    padding : Padding,
    width : Length,
    height : Length,
    maxWidth : F32,
    alignItems : Alignment,
    clip : Bool,
}

column = \children ->
    Column {
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

maxWidth = \elem, w -> update elem \c -> { c & maxWidth: w }

alignItems = \elem, a -> update elem \c -> { c & alignItems: a }

clip = \elem, b -> update elem \c -> { c & clip: b }

update = \elem, f ->
    when elem is
        Column c -> Column (f c)
        _ -> elem

