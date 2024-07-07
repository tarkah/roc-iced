platform "glue-workaround"
    requires {} { notUsed : _ }
    exposes []
    packages {}
    imports [Action.{ Action }, Length.{ Length }, Padding.{ Padding }, Element.Container, Settings.{ Settings }]
    provides [mainForHost]

Element message : [
    Text Str,
    Row (List (Element message)),
    Column (List (Element message)),
    Container { content : Element message, width : Length, height : Length, centerX : Bool, centerY : Bool, padding : Padding, style : Element.Container.Style },
    Button { content : Element message, onPress : message },
    Checkbox { label : Str, isChecked : Bool, onToggle : Bool -> message },
    TextInput { value : Str, width : Length, onInput : Str -> message, onSubmit : message },
]

# We are generating only glue for the types we need as a workaround until `roc glue`
# is able to generate correctly for the platform
GlueStuff : {
    a : Element {},
    b : Length,
    c : Action {},
    d : Settings,
}

mainForHost : GlueStuff
mainForHost = notUsed
