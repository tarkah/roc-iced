app [makeGlue] {
    pf: platform "https://github.com/lukewilliamboswell/roc/releases/download/test/olBfrjtI-HycorWJMxdy7Dl2pcbbBoJy4mnSrDtRrlI.tar.br",
}

import pf.Types exposing [Types]
import pf.File exposing [File]

makeGlue : List Types -> Result (List File) Str
makeGlue = \types ->
    types
    |> List.first
    |> Result.map \t -> [{ name: "types.txt", content: codeGenTypes t }]
    |> Result.mapErr \_ -> "No types found"

codeGenTypes : Types -> Str
codeGenTypes = \types ->

    declarations = Types.walkShapes types (List.withCapacity 1000) \decls, type, id ->
        alignment = Types.alignment types id |> Num.toStr
        size = Types.size types id |> Num.toStr

        when type is
            Struct { name, fields } ->
                fieldList =
                    when fields is
                        HasNoClosure structFields -> List.map structFields \v -> inspectField types v
                        HasClosure structFields -> List.map structFields \v -> inspectField types v

                fieldBlock = Str.joinWith fieldList "\n"

                header = "$(name) - size = $(size), align = $(alignment)\n"

                List.append decls (Str.concat header fieldBlock)

            _ ->
                # just ignore unsupported types
                decls

    "$(Str.joinWith declarations "\n\n")"

inspectField = \types, { name, id } ->
    { name, shape: Types.shape types id } |> Inspect.toStr
