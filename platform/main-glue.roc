platform "glue-workaround"
    requires {} { notUsed : _ }
    exposes []
    packages {}
    imports [Alignment, Length.{ Length }, Padding.{ Padding }, Option.{ Option }, Color.{ Color }, Border.{ Border }]
    provides [mainForHost]

# Element message : [
#     Text Str,
#     Row (List (Element message)),
#     Column (List (Element message)),
#     Container { content : Element message, width : Length, height : Length, centerX : Bool, centerY : Bool, padding : Padding, style : Element.Container.Style },
#     Button { content : Element message, onPress : message },
#     Checkbox { label : Str, isChecked : Bool, onToggle : Bool -> message },
#     TextInput { value : Str, width : Length, onInput : Str -> message, onSubmit : message },
# ]

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

Style : { textColor : Color, background : Color, border : Border }

# We are generating only glue for the types we need as a workaround until `roc glue`
# is able to generate correctly for the platform
GlueStuff : {
    a : Container {},
    # a : Alignment.Vertical,
    # b : Alignment.Horizontal,
    # c : Length,
    # d : Action {},
    # e : Settings,
}

mainForHost : GlueStuff
mainForHost = notUsed
