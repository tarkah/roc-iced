platform "glue-workaround"
    requires {} { notUsed : _ }
    exposes []
    packages {}
    imports [Alignment.{ Alignment }, Length.{ Length }, Padding.{ Padding }]
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

# We are generating only glue for the types we need as a workaround until `roc glue`
# is able to generate correctly for the platform
GlueStuff : {
    a : Column {},
    # a : Alignment.Vertical,
    # b : Alignment.Horizontal,
    # c : Length,
    # d : Action {},
    # e : Settings,
}

mainForHost : GlueStuff
mainForHost = notUsed
