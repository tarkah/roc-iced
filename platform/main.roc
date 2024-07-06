platform "iced"
    requires { Model, Message } { program : _ }
    exposes [Action, Border, Color, Element, Length, Option, Padding]
    packages {}
    imports [Element.{ Element }, Box.{ unbox, box }]
    provides [mainForHost]

# We box the model before passing to the Host and unbox when passed to Roc
ProgramForHost : {
    init : Box Model,
    update : Box Model, Box Message -> Box Model,
    view : Box Model -> Element (Box Message),
}

init : Box Model
init = box program.init

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
