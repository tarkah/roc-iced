app [program, Model, Message] {
    iced: platform "../platform/main.roc",
}

import iced.Color
import iced.Element.Button as Button
import iced.Element.Button exposing [button]
import iced.Element.Column as Column
import iced.Element.Column exposing [column]
import iced.Element.Container as Container
import iced.Element.Container exposing [container]
import iced.Element exposing [Element]
import iced.Settings exposing [Settings]

program = { init, update, view }

Model : { count : U64, isFooChecked : Bool, isBarChecked : Bool, input : Str }

Message : [
    IncrementCount,
    FooToggled Bool,
    BarToggled Bool,
    Input Str,
    Submitted,
]

init : { model : Model, settings : Settings }
init = {
    model: { count: 0, isFooChecked: Bool.false, isBarChecked: Bool.true, input: "" },
    settings: Settings.default |> Settings.size { width: 300, height: 300 },
}

update : Model, Message -> Model
update = \model, message ->
    when message is
        IncrementCount -> { model & count: model.count + 1 }
        FooToggled isFooChecked -> { model & isFooChecked }
        BarToggled isBarChecked -> { model & isBarChecked }
        Input input -> { model & input }
        Submitted -> { model & input: "" }

view : Model -> Element Message
view = \model ->
    content =
        column [
            Text "Roc + Iced <3",
            incrementButton model.count,
            Checkbox {
                label: "Foo",
                isChecked: model.isFooChecked,
                onToggle: Active FooToggled,
            },
            Checkbox {
                label: "Bar",
                isChecked: model.isBarChecked,
                onToggle: Active BarToggled,
            },
            Checkbox {
                label: "Baz",
                isChecked: Bool.false,
                onToggle: Disabled,
            },
            TextInput {
                value: model.input,
                width: Fixed 150,
                onInput: Active Input,
                onSubmit: Active Submitted,
            },
        ]
        |> Column.spacing 4

    content
    |> boxed
    |> centered

incrementButton = \count ->
    button (Text "Pressed $(Num.toStr count) times")
    |> Button.onPress IncrementCount

boxed = \elem ->
    background = Some (Color.fromHex 0xcad4e0ff)
    border = { width: 1, color: Color.black, radius: 3 }
    style = { border, background }

    elem
    |> container
    |> Container.padding 8
    |> Container.style style

centered = \elem ->
    elem
    |> container
    |> Container.center Fill
