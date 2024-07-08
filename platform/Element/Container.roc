module [
    Container,
    Style,
    container,
    padding,
    width,
    height,
    fillX,
    fillY,
    fill,
    maxWidth,
    maxHeight,
    alignX,
    alignY,
    centerX,
    centerY,
    center,
    clip,
    style,
]

import Alignment
import Color exposing [Color]
import Option exposing [Option]
import Border exposing [Border]
import Length exposing [Length]
import Padding exposing [Padding]

Container content : {
    content : content,
    padding : Padding,
    width : Option Length,
    height : Option Length,
    maxWidth : F32,
    maxHeight : F32,
    horizontalAlignment : Alignment.Horizontal,
    verticalAlignment : Alignment.Vertical,
    clip : Bool,
    style : Style,
}

container = \content ->
    Container {
        content,
        padding: Padding.zero,
        width: None,
        height: None,
        maxWidth: Num.infinityF32,
        maxHeight: Num.infinityF32,
        horizontalAlignment: Left,
        verticalAlignment: Top,
        clip: Bool.false,
        style: defaultStyle,
    }

padding = \elem, p -> update elem \c -> { c & padding: Padding.padding p }

width = \elem, w -> update elem \c -> { c & width: Some w }

height = \elem, h -> update elem \c -> { c & height: Some h }

fillX = \elem -> update elem \c -> { c & width: Some Fill }

fillY = \elem -> update elem \c -> { c & height: Some Fill }

fill = \elem -> elem |> fillX |> fillY

maxWidth = \elem, w -> update elem \c -> { c & maxWidth: w }

maxHeight = \elem, h -> update elem \c -> { c & maxHeight: h }

alignX = \elem, x -> update elem \c -> { c & horizontalAlignment: x }

alignY = \elem, y -> update elem \c -> { c & verticalAlignment: y }

centerX = \elem, length -> update elem \c -> { c & width: Some length, horizontalAlignment: Center }

centerY = \elem, length -> update elem \c -> { c & height: Some length, verticalAlignment: Center }

center = \elem, length -> elem |> centerX length |> centerY length

clip = \elem, b -> update elem \c -> { c & clip: b }

style = \elem, { textColor ? None, background ? None, border ? Border.default } ->
    update elem \c -> { c & style: { textColor, background, border } }

update = \elem, f ->
    when elem is
        Container c -> Container (f c)
        _ -> elem

Style : { textColor : Option Color, background : Option Color, border : Border }

defaultStyle = {
    textColor: None,
    background: None,
    border: Border.default,
}
