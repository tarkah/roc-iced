platform "iced"
    requires { Model, Message } { program : _ }
    exposes [
        Action,
        Alignment,
        Border,
        Color,
        Element,
        Length,
        Option,
        Padding,
        Settings,
    ]
    packages {}
    imports [PlatformElement.{ PlatformElement }, Settings.{ Settings }, Box.{ unbox, box }]
    provides [mainForHost]

# We box the model before passing to the Host and unbox when passed to Roc
ProgramForHost : {
    init : { model : Box Model, settings : Settings },
    update : Box Model, Box Message -> Box Model,
    view : Box Model -> PlatformElement (Box Message),
}

init : { model : Box Model, settings : Settings }
init =
    program.init
    |> \{ model, settings ? Settings.default } -> { model: box model, settings }

update : Box Model, Box Message -> Box Model
update = \model, message ->
    program.update (unbox model) (unbox message)
    |> box

view : Box Model -> PlatformElement (Box Message)
view = \model ->
    program.view (unbox model)
    |> PlatformElement.boxed

mainForHost : ProgramForHost
mainForHost = { init, update, view }
