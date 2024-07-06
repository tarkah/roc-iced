module [Color, default, black, white, fromHex]

Color : { r : F32, g : F32, b : F32, a : F32 }

default = fromHex 0
black = fromHex 0x000000ff
white = fromHex 0xffffffff

fromHex : U32 -> Color
fromHex =
    \hex ->
        normalize = \n -> n |> Num.toF32 |> Num.div 255

        r = hex |> Num.shiftRightZfBy 24 |> Num.bitwiseAnd 0xff |> normalize
        g = hex |> Num.shiftRightZfBy 16 |> Num.bitwiseAnd 0xff |> normalize
        b = hex |> Num.shiftRightZfBy 8 |> Num.bitwiseAnd 0xff |> normalize
        a = hex |> Num.bitwiseAnd 0xff |> normalize

        { r, g, b, a }
