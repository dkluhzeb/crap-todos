crap.collections.define("categories", {
    labels = {
        singular = "Category",
        plural = "Categories",
    },
    timestamps = true,
    admin = {
        use_as_title = "name",
        default_sort = "name",
        list_searchable_fields = { "name" },
    },
    fields = {
        crap.fields.text({
            name = "name",
            required = true,
            unique = true,
            admin = { placeholder = "Category name" },
        }),
        crap.fields.text({
            name = "color",
            default_value = "#6b7280",
            admin = {
                placeholder = "#hex",
                description = "Color for the category badge",
            },
        }),
        crap.fields.text({
            name = "icon",
            admin = {
                placeholder = "Emoji or icon name",
                description = "e.g. briefcase, house, brain, skull",
            },
        }),
    },
    access = {
        read = "access.authenticated",
        create = "access.admin_only",
        update = "access.admin_only",
        delete = "access.admin_only",
    },
})
