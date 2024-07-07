platform "iced"
    requires { Model, Message } { program : _ }
    exposes [
        Action,
        Border,
        Color,
        Element,
        Length,
        Option,
        Padding,
        Settings,
    ]
    packages {}
    imports [Element.{ Element }, Settings.{ Settings }, Box.{ unbox, box }]
    provides [mainForHost]

# We box the model before passing to the Host and unbox when passed to Roc
ProgramForHost : {
    init : { model : Box Model, settings : Settings },
    update : Box Model, Box Message -> Box Model,
    view : Box Model -> Element (Box Message),
}

init : { model : Box Model, settings : Settings }
init =
    program.init
    |> \{ model, settings ? Settings.default } -> { model: box model, settings }

update : Box Model, Box Message -> Box Model
update = \model, message ->
    program.update (unbox model) (unbox message)
    |> box

view : Box Model -> Element (Box Message)
view = \model ->
    program.view (unbox model)
    |> Element.map box

mainForHost : ProgramForHost
mainForHost = { init, update, view }
