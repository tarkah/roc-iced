platform "glue-workaround"
    requires {} { notUsed : _ }
    exposes []
    packages {}
    imports [Action.{ Action }, Element.Button, Color.{ Color }, Option.{ Option }, Border.{ Border }, Length.{ Length }, Padding.{ Padding }]
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

Style : {
    background : Option Color,
    textColor : Color,
    border : Border,
}

Button content message : {
    content : content,
    onPress : Action message,
    width : Length,
    height : Length,
    padding : Padding,
    clip : Bool,
    style : Element.Button.Status -> Style,
}

# We are generating only glue for the types we need as a workaround until `roc glue`
# is able to generate correctly for the platform
GlueStuff : {
    a : Button {} {},
    # a : Alignment.Vertical,
    # b : Alignment.Horizontal,
    # c : Length,
    # d : Action {},
    # e : Settings,
}

mainForHost : GlueStuff
mainForHost = notUsed
