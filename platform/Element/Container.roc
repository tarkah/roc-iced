module [Container, Style, container, center, padding, style]

import Color exposing [Color]
import Option exposing [Option]
import Border exposing [Border]
import Length exposing [Length]
import Padding exposing [Padding]

Container content : {
    content : content,
    width : Length,
    height : Length,
    centerX : Bool,
    centerY : Bool,
    padding : Padding,
    style : Style,
}

container = \content ->
    Container {
        content,
        width: Unspecified,
        height: Unspecified,
        centerX: Bool.false,
        centerY: Bool.false,
        padding: Padding.zero,
        style: defaultStyle,
    }

center = \elem, length ->
    when elem is
        Container c -> Container { c & width: length, height: length, centerX: Bool.true, centerY: Bool.true }
        _ -> elem

padding = \elem, p ->
    when elem is
        Container c -> Container { c & padding: Padding.padding p }
        _ -> elem

style = \elem, { textColor ? None, background ? None, border ? Border.default } ->
    when elem is
        Container c -> Container { c & style: { textColor, background, border } }
        _ -> elem

Style : { textColor : Option Color, background : Option Color, border : Border }

defaultStyle = {
    textColor: None,
    background: None,
    border: Border.default,
}
