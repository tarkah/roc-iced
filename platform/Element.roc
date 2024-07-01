module [Element, map]

import Box exposing [box, unbox]
import Action exposing [Action]
import Length exposing [Length]
import Element.Container as Container
import Element.Container exposing [Container]

Element message : [
    Text Str,
    Row (List (Element message)),
    Column (List (Element message)),
    Container (Container (Element message)),
    Button { content : Element message, onPress : Action message },
    Checkbox { label : Str, isChecked : Bool, onToggle : Action (Box (Bool -> message)) },
    TextInput { value : Str, width : Length, onInput : Action (Box (Str -> message)), onSubmit : Action message },
]

map : Element a, (a -> b) -> Element b
map = \elem, mapper ->
    elemMapper = \e -> map e mapper

    when elem is
        Text s ->
            Text s

        Row children ->
            Row (List.map children elemMapper)

        Column children ->
            Column (List.map children elemMapper)

        Container { content, width, height, centerX, centerY } ->
            Container { content: elemMapper content, width, height, centerX, centerY }

        Button { content, onPress } ->
            Button { content: elemMapper content, onPress: Action.map onPress \a -> mapper a }

        Checkbox { label, isChecked, onToggle } ->
            Checkbox {
                label,
                isChecked,
                onToggle: Action.map onToggle \a -> box (\b -> mapper ((unbox a) b)),
            }

        TextInput { value, width, onInput, onSubmit } ->
            TextInput {
                value,
                width,
                onInput: Action.map onInput \a -> box (\b -> mapper ((unbox a) b)),
                onSubmit: Action.map onSubmit \a -> mapper a,
            }
