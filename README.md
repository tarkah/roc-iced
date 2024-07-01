# Roc Iced

A [Roc](https://github.com/roc-lang/roc) plaform for building [Iced](https://github.com/iced-rs/iced) applications.

## Example

Run the basic example: `roc run examples/basic.roc`

![demo](./assets/demo.gif)

```elm
app [program, Model, Message] {
    iced: platform "../platform/main.roc",
}

import iced.Element exposing [Element]
import iced.Element.Container as Container
import iced.Element.Container exposing [container]
import Box exposing [box]

program = { init, update, view }

Model : { count : U64, isFooChecked : Bool, isBarChecked : Bool, input : Str }

Message : [
    IncrementCount,
    FooToggled Bool,
    BarToggled Bool,
    Input Str,
    Submitted,
]

init : Model
init = { count: 0, isFooChecked: Bool.false, isBarChecked: Bool.true, input: "" }

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
    Column [
        Text "Roc + Iced <3",
        Button {
            content: Text "Pressed $(Num.toStr model.count) times",
            onPress: Active IncrementCount,
        },
        Checkbox {
            label: "Foo",
            isChecked: model.isFooChecked,
            onToggle: Active (box FooToggled),
        },
        Checkbox {
            label: "Bar",
            isChecked: model.isBarChecked,
            onToggle: Active (box BarToggled),
        },
        Checkbox {
            label: "Baz",
            isChecked: Bool.false,
            onToggle: Disabled,
        },
        TextInput {
            value: model.input,
            width: Fixed 150,
            onInput: Active (box Input),
            onSubmit: Active Submitted,
        },
    ]
    |> body

body = \elem ->
    elem
    |> container
    |> Container.center Fill
```
