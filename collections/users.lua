crap.collections.define("users", {
    labels = {
        singular = "User",
        plural = "Users",
    },
    timestamps = true,
    auth = {
        forgot_password = false,
        verify_email = false,
    },
    admin = {
        use_as_title = "name",
        default_sort = "-created_at",
        list_searchable_fields = { "email", "name" },
    },
    fields = {
        crap.fields.text({
            name = "name",
            required = true,
            admin = { placeholder = "Display name" },
        }),
        crap.fields.upload({
            name = "avatar",
            relationship = { collection = "media" },
            admin = { description = "Profile picture" },
        }),
        crap.fields.textarea({
            name = "bio",
            admin = {
                rows = 3,
                placeholder = "Tell us about yourself (or don't)",
            },
        }),
        crap.fields.select({
            name = "role",
            required = true,
            default_value = "user",
            options = {
                { label = "Admin", value = "admin" },
                { label = "User", value = "user" },
            },
            admin = { position = "sidebar" },
        }),
    },
    access = {
        read = "access.authenticated",
        create = "access.admin_only",
        update = "access.self_or_admin",
        delete = "access.admin_only",
    },
})
