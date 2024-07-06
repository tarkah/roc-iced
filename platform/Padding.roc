module [Padding, padding, zero]

Padding : { top : F32, right : F32, bottom : F32, left : F32 }

zero = padding 0

padding = \f -> { top: f, right: f, bottom: f, left: f }
