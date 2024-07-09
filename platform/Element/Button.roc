module [
    Button,
    Status,
    Style,
    button,
    onPress,
    padding,
    width,
    height,
    style,
]

import Action exposing [Action]
import Border exposing [Border]
import Color exposing [Color]
import Option exposing [Option]
import Padding exposing [Padding]
import Length exposing [Length]

Button content message : {
    content : content,
    onPress : Action message,
    width : Option Length,
    height : Option Length,
    padding : Padding,
    clip : Bool,
    style : Box (Status -> Option Style),
}

button = \content ->
    Button {
        content,
        onPress: Disabled,
        width: None,
        height: None,
        padding: defaultPadding,
        clip: Bool.false,
        style: Box.box \_ -> None,
    }

onPress = \elem, a -> update elem \c -> { c & onPress: Active a }

padding = \elem, p -> update elem \c -> { c & padding: Padding.padding p }

width = \elem, w -> update elem \c -> { c & width: Some w }

height = \elem, h -> update elem \c -> { c & height: Some h }

style = \elem, f ->
    withDefault = \status ->
        s = f status
        when s is
            Some { background ? None, textColor ? Color.black, border ? Border.default } -> Some { background, textColor, border }
            None -> None

    update elem \c -> { c & style: Box.box withDefault }

update = \elem, f ->
    when elem is
        Button c -> Button (f c)
        _ -> elem

defaultPadding = { top: 5, bottom: 5, right: 10, left: 10 }

Status : [
    Active,
    Hovered,
    Pressed,
    Disabled,
]

Style : {
    background : Option Color,
    textColor : Color,
    border : Border,
}
