crap.collections.define("comments", {
    labels = {
        singular = "Comment",
        plural = "Comments",
    },
    timestamps = true,
    admin = {
        use_as_title = "body",
        default_sort = "-created_at",
    },
    fields = {
        crap.fields.textarea({
            name = "body",
            required = true,
            admin = {
                rows = 3,
                placeholder = "Add your two cents",
            },
        }),
        crap.fields.relationship({
            name = "author",
            required = true,
            relationship = {
                collection = "users",
                has_many = false,
            },
        }),
        crap.fields.relationship({
            name = "task",
            required = true,
            relationship = {
                collection = "tasks",
                has_many = false,
            },
        }),
    },
    access = {
        read = "access.authenticated",
        create = "access.authenticated",
        update = "access.author_or_admin",
        delete = "access.author_or_admin",
    },
})
