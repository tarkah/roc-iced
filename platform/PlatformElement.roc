module [PlatformElement, boxed]

import Box exposing [box]

import Action exposing [Action]
import Element exposing [Element]
import Element.Column exposing [Column]
import Element.Container exposing [Container]
import Element.Row exposing [Row]
import Length exposing [Length]

PlatformElement message : [
    Text Str,
    Row (Row (PlatformElement message)),
    Column (Column (PlatformElement message)),
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

            Row { children, spacing, padding, width, height, alignItems, clip } ->
                Row { children: List.map children map, spacing, padding, width, height, alignItems, clip }

            Column { children, spacing, padding, width, height, maxWidth, alignItems, clip } ->
                Column { children: List.map children map, spacing, padding, width, height, maxWidth, alignItems, clip }

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
