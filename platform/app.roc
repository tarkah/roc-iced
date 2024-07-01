app [program, Model, Message] {
    iced: platform "main.roc",
}

Model : {}
Message : {}

program = {
    init: {},
    update: \m, _ -> m,
    view: \_ -> Text "Hello World!",
}
