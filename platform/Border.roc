module [Border, Radius, default, rounded]

import Color exposing [Color]

Border : { color : Color, width : F32, radius : Radius }

default = { color: Color.default, width: 0, radius: 0 }

rounded = \radius -> { default & radius: Num.toF32 radius }

# TODO: Tuple of corners
Radius : F32

