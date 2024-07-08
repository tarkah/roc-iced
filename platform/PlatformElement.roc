module [PlatformElement, boxed]

import Box exposing [box]

import Action exposing [Action]
import Element exposing [Element]
import Element.Container exposing [Container]
import Length exposing [Length]

PlatformElement message : [
    Text Str,
    Row (List (PlatformElement message)),
    Column (List (PlatformElement message)),
    Container (Container (PlatformElement message)),
    Button { content : PlatformElement message, onPress : Action message },
    Checkbox { label : Str, isChecked : Bool, onToggle : Action (Box (Bool -> message)) },
    TextInput { value : Str, width : Length, onInput : Action (Box (Str -> message)), onSubmit : Action message },
]

boxed : Element a -> PlatformElement (Box a)
boxed = \elem ->
    map = \e ->
        when e is
            Text s ->
                Text s

            Row children ->
                Row (List.map children map)

            Column children ->
                Column (List.map children map)

            Container { content, padding, width, height, maxWidth, maxHeight, horizontalAlignment, verticalAlignment, clip, style } ->
                Container { content: map content, padding, width, height, maxWidth, maxHeight, horizontalAlignment, verticalAlignment, clip, style }

            Button { content, onPress } ->
                Button { content: map content, onPress }

            Checkbox { label, isChecked, onToggle } ->
                Checkbox {
                    label,
                    isChecked,
                    onToggle: Action.map onToggle box,
                }

            TextInput { value, width, onInput, onSubmit } ->
                TextInput {
                    value,
                    width,
                    onInput: Action.map onInput box,
                    onSubmit,
                }

    elem |> Element.map box |> map
