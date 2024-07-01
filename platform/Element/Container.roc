module [Container, container, center]

import Length exposing [Length]

Container content : { content : content, width : Length, height : Length, centerX : Bool, centerY : Bool }

container = \content ->
    Container { content, width: Unspecified, height: Unspecified, centerX: Bool.false, centerY: Bool.false }

center = \elem, length ->
    when elem is
        Container c -> Container { c & width: length, height: length, centerX: Bool.true, centerY: Bool.true }
        _ -> elem
