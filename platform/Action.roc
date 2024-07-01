module [Action, map]

Action a : [Disabled, Active a]

map : Action a, (a -> b) -> Action b
map = \action, mapper ->
    when action is
        Disabled -> Disabled
        Active a -> Active (mapper a)
