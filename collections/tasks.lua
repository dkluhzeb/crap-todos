crap.collections.define("tasks", {
    labels = {
        singular = "Task",
        plural = "Tasks",
    },
    timestamps = true,
    versions = true,
    live = true,
    soft_delete = true,
    admin = {
        use_as_title = "title",
        default_sort = "-created_at",
        list_searchable_fields = { "title" },
    },
    fields = {
        crap.fields.text({
            name = "title",
            required = true,
            admin = { placeholder = "What needs doing?" },
            hooks = {
                before_validate = { "hooks.trim_title" },
            },
        }),
        crap.fields.text({
            name = "slug",
            unique = true,
            admin = {
                readonly = true,
                position = "sidebar",
                description = "Auto-generated from title",
            },
            hooks = {
                before_validate = { "hooks.auto_slug" },
            },
        }),
        crap.fields.richtext({
            name = "description",
            admin = {
                placeholder = "Elaborate on your procrastination",
            },
        }),
        crap.fields.row({
            name = "status_priority_row",
            fields = {
                crap.fields.select({
                    name = "status",
                    required = true,
                    default_value = "barely_started",
                    options = {
                        { label = "Barely Started", value = "barely_started" },
                        { label = "Kinda Doing It", value = "kinda_doing_it" },
                        { label = "Almost Done", value = "almost_done" },
                        { label = "Done (Allegedly)", value = "done_allegedly" },
                    },
                    admin = { width = "half" },
                }),
                crap.fields.number({
                    name = "priority",
                    required = true,
                    default_value = 1,
                    min = 1,
                    max = 5,
                    admin = {
                        width = "half",
                        description = "1-5 (displayed as poop emojis)",
                        step = "1",
                    },
                }),
            },
        }),
        crap.fields.row({
            name = "dates_row",
            fields = {
                crap.fields.date({
                    name = "due_date",
                    admin = {
                        width = "half",
                        description = "When guilt kicks in",
                    },
                }),
                crap.fields.checkbox({
                    name = "overdue",
                    default_value = false,
                    admin = {
                        width = "half",
                        readonly = true,
                        description = "Auto-computed from due date",
                    },
                }),
            },
        }),
        crap.fields.relationship({
            name = "assignee",
            relationship = {
                collection = "users",
                has_many = false,
            },
            admin = {
                position = "sidebar",
                description = "Who's responsible for this mess",
            },
        }),
        crap.fields.relationship({
            name = "category",
            relationship = {
                collection = "categories",
                has_many = false,
            },
            admin = {
                position = "sidebar",
            },
        }),
        crap.fields.upload({
            name = "attachments",
            relationship = {
                collection = "media",
                has_many = true,
            },
            admin = {
                description = "Evidence of your procrastination",
            },
        }),
        crap.fields.array({
            name = "subtasks",
            admin = {
                label_field = "title",
                labels = {
                    singular = "Subtask",
                    plural = "Subtasks",
                },
                description = "Break your failures into smaller failures",
            },
            fields = {
                crap.fields.text({
                    name = "title",
                    required = true,
                    admin = { placeholder = "Sub-failure description" },
                }),
                crap.fields.checkbox({
                    name = "done",
                    default_value = false,
                }),
            },
        }),
    },
    hooks = {
        before_change = { "hooks.check_overdue" },
    },
    access = {
        read = "access.authenticated",
        create = "access.authenticated",
        update = "access.authenticated",
        delete = "access.authenticated",
        trash = "access.authenticated",
    },
})
