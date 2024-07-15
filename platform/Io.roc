module [println]

import Effect
import InternalTask
import Task exposing [Task]

println : Str -> Task {} *
println = \s ->
    Effect.println s
    |> Effect.map Ok
    |> InternalTask.fromEffect
